use crate::{
	import_queue, utils,
    AuthorityId, authorities,
	MAX_VOTE_RANK, PROPOSAL_TIMEOUT, AUTHOR_S0_TIMEOUT, AUTHOR_S1_TIMEOUT,
	// ElectionWeightInfo,
	ElectionInfoByHeader,
};

use codec::{Decode, Encode};
use futures::{channel::mpsc, prelude::*};
use futures_timer::Delay;
use std::{
	convert::{TryFrom, TryInto},
    sync::Arc,
    fmt::Debug,
    time::{SystemTime, Duration},
	pin::Pin,
};
// use schnorrkel::vrf::{VRFOutput};
use schnorrkel::keys::PublicKey;

use sc_client_api::{
	BlockchainEvents, BlockOf, 
};

use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr, vrf::VRFSignature};

use sp_core::crypto::{Pair, Public};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppKey, AppPublic};
use sp_blockchain::{HeaderBackend};
use sp_inherents::{CreateInherentDataProviders, InherentDataProvider};
use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT},
	DigestItem,
};

use sp_consensus::{
    SelectChain, SyncOracle, VELink,
	VoteElectionRequest, VoteData, ElectionData, Environment,
	Proposer, BlockOrigin,
};

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	Slot,
	VoteElectionApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};

use sc_consensus::{BlockImport, BlockImportParams, StateAction, ForkChoiceStrategy};
use sc_telemetry::{TelemetryHandle};

use crate::worker::{InherentDataProviderExt, StorageChanges};

pub use import_queue::{
	build_verifier, AuraVerifier, BuildVerifierParams, CheckForEquivocation,
	ImportQueueParams,
};


#[derive(Clone)]
pub struct StateInfo<B: BlockT, P: Pair>{
	cur_header: B::Header,
	cur_weight: u64,
	min_weight: u64,
	max_weight: u64,
	pub_bytes: Vec<u8>,
	committee_vec: Vec<AuthorityId<P>>,
	election_vec: Vec<ElectionData<B>>,
}

pub struct AuthorWorker<P, B, C, E, I, SO, VL, CIDP>
where
	B: BlockT,
	P: Pair,
{
	pub client: Arc<C>,
	pub block_import: I,
	pub env: E,
	pub sync_oracle: SO,
	pub force_authoring: bool,
	pub keystore: SyncCryptoStorePtr,
	pub telemetry: Option<TelemetryHandle>,
	pub create_inherent_data_providers: CIDP,
	pub vote_link: VL,
	pub state_info: Option<StateInfo<B, P>>,
}


impl<P, B, C, E, I, SO, VL, CIDP, Error> AuthorWorker<P, B, C, E, I, SO, VL, CIDP> 
where
	B: BlockT,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: TryFrom<Vec<u8>> + Encode + Decode,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + BlockOf + Sync + Send + 'static, 
	C::Api: VoteElectionApi<B, AuthorityId<P>>,
	E: Environment<B, Error = Error> + Send + Sync,
	E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	VL: VELink<B> + Send + Clone,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	SO: SyncOracle<B> + Send + Sync + Clone,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
{

	/// Reset the state, including data which will be used in a vote-election period
	pub fn reset_state_info(&mut self, cur_header: &B::Header)->Result<(), String>{
		// basic state_info
		let committee_vec = authorities(self.client.as_ref(), &BlockId::Hash(cur_header.hash()))
			.map_err(|e|format!("get pallet authorities failed: {}", e))?;

		let (min_weight, max_weight) = 
			utils::caculate_min_max_election_weight(committee_vec.len(), MAX_VOTE_RANK);

		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, sp_application_crypto::key_types::VOTE);

		if sr25519_public_keys.len() == 0{
			Err(format!("no public key"))?
		}
		let pub_bytes = sr25519_public_keys[0].to_raw_vec();
		
		self.state_info = Some(StateInfo{
			cur_header: cur_header.clone(),
			cur_weight: max_weight,
			min_weight,
			max_weight,
			pub_bytes,
			committee_vec,
			election_vec: vec![],
		});
		Ok(())
	}

	/// Generate vrf data and propagate the vote
    fn generate_vrf_and_propagate(&mut self, cur_header: &B::Header)->Result<(u128, VRFSignature), String>{
		let state_info = self.state_info.as_ref().ok_or(format!("no state info"))?;

		let (vrf_num, vrf_sig) = match self.generate_vrf_data(&cur_header.hash()){
			Ok(v)=>{
				log::info!(
					"Author.S1, gen vrf u128: 0x{:0>32X}, #{} ({})",
					v.0, cur_header.number(), cur_header.hash(),
				);
				v
			},
			Err(e)=>{
				Err(format!("Author: generate vrf failed: {}", e))?
			},
		};

		let vote_data = VoteData::<B>{
			block_hash: cur_header.hash().clone(),
			vrf_output_bytes: vrf_sig.output.to_bytes().encode(),
			vrf_proof_bytes: vrf_sig.proof.to_bytes().encode(),
			pub_bytes: state_info.pub_bytes.clone(),
		};
		self.vote_link.ve_request(VoteElectionRequest::PropagateVote(vote_data));

		Ok((vrf_num, vrf_sig))
    }

	/// Return the vrf data base at the block hash
	fn generate_vrf_data(&self, cur_hash: &B::Hash)->Result<(u128, VRFSignature), String>{
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::VOTE
		);

		if sr25519_public_keys.len() == 0{
			return Err("Public key count not 1".to_string());
		}

		let msg = cur_hash.clone().encode();
		let transcript = make_transcript(&msg);
		let transcript_data = make_transcript_data(&msg);

		if let Ok(Some(vrf_sig)) = SyncCryptoStore::sr25519_vrf_sign(
			&*self.keystore,
			<AuthorityId<P> as AppKey>::ID,
			&sr25519_public_keys[0],
			transcript_data,
		){
			let public = PublicKey::from_bytes(&sr25519_public_keys[0].to_raw_vec())
				.map_err(|e|format!("Decode public key failed: {}", e))?;

			match vrf_sig.output.attach_input_hash(&public, transcript){
				Ok(inout)=>{
					let vrf_num = u128::from_le_bytes(inout.make_bytes::<[u8; 16]>(VOTE_VRF_PREFIX));
					return Ok((vrf_num, vrf_sig));
				},
				Err(e)=>{
					return Err(format!("Recover vrf failed: {}", e));
				}
			}
		}
		else{
			Err("VRF signature failed".to_string())
		}
	}

	/// Retain the election result and update the proposal weight 
    fn recv_election_and_update_weight(&mut self, election: &ElectionData<B>)->Result<u64, String>{
		let state_info = self.state_info.as_ref().ok_or(format!("state info not exist"))?;
		if election.block_hash != state_info.cur_header.hash(){
			log::info!(
				"Author: election hash dismatch: cur: #{} ({}), recv: {}",
				state_info.cur_header.number(),
				state_info.cur_header.hash(),
				election.block_hash,
			);
			return Err(format!("Invalid electon"));
		}

		if let Err(e) = self.verify_election(&election){
			log::info!("Author:, election verify failed: {}", e);
			return Err(format!("Election verify failed"));
		}

		let state_info = self.state_info.as_mut().ok_or(format!("no state info"))?;
		state_info.election_vec.push(election.clone());

		let cur_weight = utils::caculate_weight_from_elections_with_output(
			&state_info.pub_bytes,
			&state_info.election_vec,
			state_info.committee_vec.len(),
			MAX_VOTE_RANK
		);
		state_info.cur_weight = cur_weight;
		Ok(cur_weight)
    }

	/// Verify the election validation
	fn verify_election(&self, election_data: &ElectionData<B>)->Result<(), String>{
		let state_info = self.state_info.as_ref().ok_or(format!("no state info"))?;

		// verify block_hash 
		if state_info.cur_header.hash() != election_data.block_hash{
			Err(format!(
				"Invalid election: expect: #{}({}), recv: {}",
				state_info.cur_header.number(),
				state_info.cur_header.hash(),
				election_data.block_hash,
			))?;
		}

		// verify committee member validation
		let mut is_committee_member = false;
		for committee in state_info.committee_vec.iter(){
			if election_data.committee_pub_bytes == committee.to_raw_vec(){
				is_committee_member |= true; 
				break;
			}
		}
		if is_committee_member == false{
			Err(format!("Not in committee members"))?
		}

		// verify signature
		let ElectionData{ block_hash, sig_bytes, vote_list, committee_pub_bytes } = election_data;
		match <P::Signature as Decode>::decode(&mut sig_bytes.as_slice()){
			Ok(sig)=>{
				let mut msg_bytes :Vec<u8> = vec![];
				msg_bytes.extend(block_hash.encode().iter());
				msg_bytes.extend(vote_list.encode().iter());

				let msg = msg_bytes.as_slice();

				match <AuthorityId<P> as Decode>::decode(&mut committee_pub_bytes.as_slice()){
					Ok(verify_public) =>{
						if P::verify(&sig, &msg, &verify_public)== true{
							return Ok(());
						}
						else{
							Err(format!("Author: election signature verify failed"))?
						}
					},
					Err(e)=>{
						Err(format!("Author: decode election public key failed: {}", e))?
					}
				}
			},
			Err(e)=>{
				Err(format!("Author: decode election signature bytes failed: {}", e))?
			}
		}
	}

	/// Returns an object with claim data if has key
	fn claim(
		&mut self,
		vrf_sig: &VRFSignature,
		election_vec: Vec<ElectionData<B>>,
	) -> Option<(PreDigest, P::Public)> {
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::VOTE
		);

		if sr25519_public_keys.len() == 1{

			let pub_bytes = sr25519_public_keys[0].to_raw_vec();
			if let Ok(author) = <AuthorityId<P> as Decode>::decode(&mut pub_bytes.as_slice()){
				let pre_digest = PreDigest{
					pub_key_bytes: author.to_raw_vec(),
					vrf_output_bytes: vrf_sig.output.to_bytes().encode(),
					vrf_proof_bytes: vrf_sig.proof.to_bytes().encode(),
					election_bytes: election_vec.encode()
				};
				return Some((pre_digest, author.clone()));
			}
		}

		None
	}

	/// Return the pre digest data to include in a block authored with the given claim.
	fn pre_digest_data(
		&self,
		claim: &(PreDigest, P::Public),
	) -> Vec<sp_runtime::DigestItem<B::Hash>> {
		vec![<DigestItem<B::Hash> as CompatibleDigestItem<P::Signature>>::ve_pre_digest(claim.0.clone())]
	}

	/// Returns the authorities
	fn get_authorities(
		&self,
		header: &B::Header,
	) -> Result<Vec<AuthorityId<P>>, sp_consensus::Error> {
		authorities(self.client.as_ref(), &BlockId::Hash(header.hash()))
	}

	/// Returns a function which produces a `BlockImportParams`.
	fn block_import_params(
		&self,
	) -> Box<
		dyn Fn(
				B::Header,
				&B::Hash,
				Vec<B::Extrinsic>,
				StorageChanges<sp_api::TransactionFor<C, B>, B>,
				(PreDigest, P::Public),
				Vec<AuthorityId<P>>,
			) -> Result<
				sc_consensus::BlockImportParams<B, sp_api::TransactionFor<C, B>>,
				sp_consensus::Error,
			> + Send
			+ 'static,
	> {
		let keystore = self.keystore.clone();
		Box::new(move |header, header_hash, body, storage_changes, (_, public), _epoch| {
			// sign the pre-sealed hash of the block and then
			// add it to a digest item.
			let public_type_pair = public.to_public_crypto_pair();
			let public = public.to_raw_vec();
			let signature = SyncCryptoStore::sign_with(
				&*keystore,
				<AuthorityId<P> as AppKey>::ID,
				&public_type_pair,
				header_hash.as_ref(),
			)
			.map_err(|e| sp_consensus::Error::CannotSign(public.clone(), e.to_string()))?
			.ok_or_else(|| {
				sp_consensus::Error::CannotSign(
					public.clone(),
					"Could not find key in keystore.".into(),
				)
			})?;
			let signature = signature
				.clone()
				.try_into()
				.map_err(|_| sp_consensus::Error::InvalidSignature(signature, public))?;

			let signature_digest_item =
				<DigestItem<B::Hash> as CompatibleDigestItem<P::Signature>>::ve_seal(signature);

			let mut import_block = BlockImportParams::new(BlockOrigin::Own, header);
			import_block.post_digests.push(signature_digest_item);
			import_block.body = Some(body);
			import_block.state_action =
				StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
			import_block.fork_choice = Some(ForkChoiceStrategy::LongestChain);

			Ok(import_block)
		})
	}

	/// Returns a `Proposer` to author on top of the given block.
	fn proposer(&mut self, header: &B::Header) -> 
		Pin<Box<dyn Future<Output = Result<E::Proposer, sp_consensus::Error>> + Send + 'static>>
	{
		Box::pin(
			self.env
				.init(header)
				.map_err(|e| sp_consensus::Error::ClientImport(format!("{:?}", e)).into()),
		)
	}

	/// Build block
	async fn proposal_block(&mut self, vrf_sig: VRFSignature)->Result<(), String>{
		let state_info = self.state_info.clone().ok_or(format!("no state info"))?;

		if state_info.cur_weight < state_info.max_weight{
			log::info!(
				target: "ve-consensus",
				"Author.S1: timeout, prepare block at: #{} ({})",
				state_info.cur_header.number(),
				state_info.cur_header.hash(),
			);

			// build block
			let proposing_remaining_duration = Duration::from_millis(PROPOSAL_TIMEOUT);

			let parent_header = state_info.cur_header.clone();

			let authorities = match self.get_authorities(&parent_header) {
				Ok(epoch_data) => epoch_data,
				Err(err) => {
					log::warn!(
						target: "ve-consensus",
						"Unable to fetch epoch data at block {:?}: {:?}",
						parent_header.hash(),
						// slot_info.chain_head.hash(),
						err,
					);

					return Err(format!("Unable to get epoch data: {:?}", err))?;
				},
			};

			let authorities_len = authorities.len();

			if !self.force_authoring &&
				self.sync_oracle.is_offline() &&
				(authorities_len > 1)
			{
				log::debug!(target: "ve-consensus", "Skipping proposal. Waiting for the network.");
				return Err("skip proposal")?;
			}

			let claim = self.claim(&vrf_sig, state_info.election_vec).ok_or("Get claim failed")?;
			let logs = self.pre_digest_data(&claim);

			let proposer = match self.proposer(&parent_header).await {
				Ok(p) => p,
				Err(err) => {
					log::warn!(
						target: "ve-consensus",
						"Unable to author block prev: {}: {:?}", parent_header.hash(), err,
					);

					return Err(format!("Unable to author block prev: {}", err))?;
				},
			};

			// deadline our production to 98% of the total time left for proposing. As we deadline
			// the proposing below to the same total time left, the 2% margin should be enough for
			// the result to be returned.
			let inherent_data_providers = self
				.create_inherent_data_providers
				.create_inherent_data_providers(state_info.cur_header.hash(), ())
				.await
				.map_err(|e|format!("Create inherent provider failed: {}", e))?;

			let inherent_data = inherent_data_providers
				.create_inherent_data()
				.map_err(|e|format!("crate inherent data failed: {}", e))?;

			let proposing = proposer
				.propose(
					inherent_data,
					sp_runtime::generic::Digest { logs },
					proposing_remaining_duration.mul_f32(0.98),
					None,
				)
				.map_err(|e| sp_consensus::Error::ClientImport(format!("{:?}", e)));

			let proposal = match proposing.await{
				Ok(p) => p,
				Err(err) => {
					log::warn!(target: "ve-consensus", "Proposing failed: {:?}", err);
					return Err(format!("proposing failed: {}", err))?;
				}
			};

			let (block, _storage_proof) = (proposal.block, proposal.proof);
			let (header, body) = block.deconstruct();
			let header_num = *header.number();
			let header_hash = header.hash();

			let block_import_params_maker = self.block_import_params();
			let block_import_params = match block_import_params_maker(
				header,
				&header_hash,
				body.clone(),
				proposal.storage_changes,
				claim,
				authorities,
			) {
				Ok(bi) => bi,
				Err(err) => {
					log::warn!(
						target: "ve-consensus",
						"Failed to create block import params: {:?}",
						err
					);
					return Err(format!("failed to create block import params: {:?}", err))?;
				},
			};

			log::info!(
				target: "ve-consensus",
				"🔖 Pre-sealed block at {}. Hash now {}, previously {}.",
				header_num,
				block_import_params.post_hash(),
				header_hash,
			);

			self.block_import
				.import_block(block_import_params, Default::default())
				.await
				.map_err(|e|format!("Import block failed: {}", e))?;
		}
		else{
			log::info!(
				target: "ve-consensus",
				"Author.S1: timeout, no weight prepare block at: #{} ({})",
				state_info.cur_header.number(),
				state_info.cur_header.hash(),
			);
		}
		Ok(())
	}
}

impl<P, B, C, E, I, SO, VL, CIDP, Error> ElectionInfoByHeader<B,P,C> for
	AuthorWorker<P, B, C, E, I, SO, VL, CIDP> 
where
	B: BlockT,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: TryFrom<Vec<u8>> + Encode + Decode,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + BlockOf + Sync + Send + 'static, 
	C::Api: VoteElectionApi<B, AuthorityId<P>>,
	E: Environment<B, Error = Error> + Send + Sync,
	E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	VL: VELink<B> + Send + Clone,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	SO: SyncOracle<B> + Send + Sync + Clone,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
{
	fn client(&self)->&C{
		self.client.as_ref()
	}
}

pub async fn run_author_worker<P, B, C, I, SO, SC, PF, VL, CIDP, Error>(
	client: Arc<C>,
	mut author: AuthorWorker<P, B, C, PF, I, SO, VL, CIDP>,
	select_chain: SC,
	mut sync_oracle: SO,
	mut vote_link: VL,
)
where
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: TryFrom<Vec<u8>> + Encode + Decode,
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + BlockOf + Sync + Send + 'static, 
	C::Api: VoteElectionApi<B, AuthorityId<P>>,
	VL: VELink<B> + Send + Clone,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	SC: SelectChain<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
    enum AuthorState<H>{
        WaitStart,
        WaitProposal(H)
    }

	// build connection with network
	let (election_tx, mut election_rx) = mpsc::unbounded();
	vote_link.ve_request(VoteElectionRequest::BuildElectionStream(election_tx));
	let mut imported_blocks_stream = client.import_notification_stream().fuse();

    let mut state = AuthorState::WaitStart;

	// fsm
    'outer: loop{
        match state{
            AuthorState::WaitStart=>{
				log::info!("► AuthorState::S0, wait block or timeout");
                let mut delay = Delay::new(Duration::from_secs(AUTHOR_S0_TIMEOUT));
                let timeout = &mut delay;

                loop{
                    futures::select!{
                        block = imported_blocks_stream.next()=>{
                            if let Some(block) = block{
                                log::info!("Author.S0, import block: #{} ({})", block.header.number(), block.hash);
                                if sync_oracle.is_major_syncing(){
                                    state = AuthorState::WaitStart;
									continue 'outer;
                                    // break;
                                }

                                state = AuthorState::WaitProposal(block.header);
                                continue 'outer;
                            }
                        },
                        _ = election_rx.select_next_some()=>{
                            continue;
                        },
                        _ = timeout.fuse()=>{
                            log::info!("Author.S0: timeout");
							let chain_head = match select_chain.best_chain().await{
								Ok(x)=>x,
								Err(e)=>{
									log::warn!("Author.S0: select_chain err: {}", e);
									state = AuthorState::WaitStart;
                                    continue 'outer;
								}
							};

							state = AuthorState::WaitProposal(chain_head);
                            continue 'outer;
                        },
                    }
                }
            },
            AuthorState::WaitProposal(parent_header)=>{
				log::info!(
					"► AuthorState::S1 #{} ({}), propagate vote and wait proposal",
					parent_header.number(),
					parent_header.hash(),
				);
				if let Err(e) = author.reset_state_info(&parent_header){
					log::warn!("Author.S1: reset state info err, {}", e);
					state = AuthorState::WaitStart;
					continue 'outer;
				}

				let (local_vrf_num, vrf_sig) = match author.generate_vrf_and_propagate(&parent_header){
					Ok(x)=>x,
					Err(e)=>{
						log::warn!("Author.S1: propagate vote err: {}", e);
						state = AuthorState::WaitStart;
						continue 'outer;
					}
				};

				let (min_weight, max_weight) = match author.state_info.as_ref(){
					Some(v)=>{
						(v.min_weight, v.max_weight)
					},
					None=>{
						log::warn!("Author.S1: get min max weight failed");
						state = AuthorState::WaitStart;
						continue 'outer;
					}
				};

				let parent_block_election_info = match author.caculate_election_info_from_header(&parent_header){
					Ok(v)=>v,
					Err(e) => {
						log::info!("Author.S1, caculate block election weight error, {:?}, #{} ({})",
							e, parent_header.number(), parent_header.hash()
						);
						state = AuthorState::WaitStart;
						continue 'outer;
					},
				};
				let parent_block_weight = parent_block_election_info.weight;

				let full_timeout_duration = Duration::from_secs(AUTHOR_S1_TIMEOUT);
				let start_time = SystemTime::now();
				let mut rest_timeout_rate = 1f32;
				let mut min_weight_delay_count = 0;
				let mut not_min_weight_delay_count = 0;

                loop{
					let timeout = update_s1_timeout(
						rest_timeout_rate,
						&start_time,
						&full_timeout_duration,
					);
                    futures::select!{
                        block = imported_blocks_stream.next()=>{
                            if let Some(block) = block{
                                log::info!("Author.S1, import block: #{} ({})", block.header.number(), block.hash);

								if sync_oracle.is_major_syncing(){
									state = AuthorState::WaitStart;
									continue 'outer;
								}

								let incoming_block_election_info = match author.caculate_election_info_from_header(&block.header){
									Ok(v)=>v,
									Err(e) => {
										log::info!("Author.S1, caculate block election weight error, {:?}, #{} ({})",
											e, block.header.number(), block.header.hash()
										);
										continue
									},
								};

								if incoming_block_election_info.exceed_half{
									log::info!(
										"Author.S1, import block #{} ({}) from outside with exceed 50% election", 
										block.header.number(),
										block.hash
									);

									if *block.header.parent_hash() != parent_header.hash(){
										log::warn!(
											"Author.S1, AS1#01 need handle: #{}({}) is not the next block for current block #{}({})",
											block.header.number(),
											block.header.hash(),
											parent_header.number(),
											parent_header.hash(),
										);
									}
									state = AuthorState::WaitProposal(block.header);
									continue 'outer;
								}

								// import block for next height
								if block.header.parent_hash() == &parent_header.hash(){
									if incoming_block_election_info.vrf_num < local_vrf_num {
										log::info!(
											"Author.S1, block #{}({}) outside with smaller random",
											block.header.number(),
											block.hash,
										);
										state = AuthorState::WaitProposal(block.header);
										continue 'outer;
									}
									else{
										log::info!(
											"Author.S1, ignore block because local vrf smaller, local: 0x{:0>32X} < 0x{:0>32X}",
											local_vrf_num,
											incoming_block_election_info.vrf_num,
										);
									}
								}
								// import block with same height
								else if block.header.parent_hash() == parent_header.parent_hash(){
									if incoming_block_election_info.weight < parent_block_weight{
										log::info!("Author.S1: change to a block with less weight, #{}({})",
											block.header.number(), block.hash) ;
										state = AuthorState::WaitProposal(block.header);
										continue 'outer;
									}
									else{
										log::info!(
											"Author.S1: ignore block with larger weight, #{}({}), cur: {}, new: {}",
											block.header.number(),
											block.hash,
											parent_block_weight,
											incoming_block_election_info.weight,
										);
									}
								}
								else{
									log::warn!(
										"Author.S1, AS1#02 need handle: cur: #{}({}), new: #{}({})",
										parent_header.number(),
										parent_header.hash(),
										block.header.number(),
										block.header.hash(),
									);
								}

								log::info!(
									"Author.S1: ignore the block: #{} ({})",
									block.header.number(),
									block.hash
								);
                                continue;
                            }
                        },
                        election = election_rx.select_next_some()=>{
							// log::info!("Author.S1, recv election");
							if let Ok(cur_weight) = author.recv_election_and_update_weight(&election){
								rest_timeout_rate = update_author_s1_timeout_rate(
									&cur_weight,
									&min_weight,
									&max_weight,
									&mut min_weight_delay_count,
									&mut not_min_weight_delay_count,
								);
								continue;
							}
                        },
                        _ = timeout.fuse()=>{
							log::info!("Author.S1, timeout");
							match author.proposal_block(vrf_sig).await{
								Ok(_)=>{},
								Err(e)=>{
									log::info!("Author.S1, propsoal block failed: {}", e);
								}
							}

							state = AuthorState::WaitStart;
							continue 'outer;
                        },
                    }
                }
            },
        }
    }

	fn update_s1_timeout(rest_timeout_rate: f32, start_time: &SystemTime, full_timeout_duration: &Duration)->Delay{
		let elapsed_duration = start_time.elapsed().unwrap_or(Duration::from_secs(0));
		let rest_timeout_duration = full_timeout_duration.checked_sub(elapsed_duration).unwrap_or(Duration::from_secs(0));
		let rest_duration_millis = rest_timeout_duration.as_millis();

		let duration = {
			if rest_timeout_rate < 1f32 {
				Duration::from_millis(((rest_duration_millis as f32)*rest_timeout_rate) as u64)
			}
			else{
				Duration::from_millis(rest_duration_millis as u64)
			}
		};
		return Delay::new(duration);
	}

	fn update_author_s1_timeout_rate(
		cur_election_weight: &u64,
		min_election_weight: &u64,
		max_election_weight: &u64,
		min_weight_delay_count: &mut u64,
		not_min_weight_delay_count: &mut u64,
	)->f32
	{
		let rest_timeout_rate = {
			if cur_election_weight <= min_election_weight{
				if *min_weight_delay_count < 10 { 
					*min_weight_delay_count += 1;
					0.015f32
				}
				else {
					0.0
				}
			}
			else{
				let rate = (cur_election_weight - min_election_weight) as f32 /
					(max_election_weight - min_election_weight) as f32;
				
				if *not_min_weight_delay_count < 10{
					*not_min_weight_delay_count += 1;
					rate + 0.66f32
				}
				else{
					rate
				}
			}
		};

		return rest_timeout_rate;
	}
}
