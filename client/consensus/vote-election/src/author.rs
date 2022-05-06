use crate::{
	slot_worker, import_queue, utils,
    AuthorityId, authorities, MAX_VOTE_RANK, COMMITTEE_TIMEOUT, PROPOSAL_TIMEOUT,
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

use sc_client_api::{
	BlockchainEvents, BlockOf, 
	// UsageProvider,
	// backend::{AuxStore, Backend as ClientBackend, Finalizer},
	// BlockchainEvents, ImportNotifications, BlockOf, FinalityNotification,
};

use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr, vrf::VRFSignature};

use sp_core::crypto::{Pair, Public};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppKey, AppPublic};
use sp_blockchain::{HeaderBackend};
use sp_inherents::{CreateInherentDataProviders, InherentDataProvider};
use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT, NumberFor},
	// traits::{Block as BlockT, HashFor, Header as HeaderT, Zero},
	DigestItem,
};

use sp_consensus::{
    SelectChain, SyncOracle, VELink as VoteLink,
    // CanAuthorWith, Proposer, SelectChain, SlotData, SyncOracle, VELink as VoteLink,
	VoteElectionRequest, VoteData, ElectionData, Environment,
	Proposer, BlockOrigin,
};

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	Slot,
	// inherents::{InherentDataProvider, InherentType as AuraInherent, INHERENT_IDENTIFIER},
	AuraApi as VoteApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};
// use sp_consensus_slots::Slot;

use sc_consensus::{BlockImport, BlockImportParams, StateAction, ForkChoiceStrategy};
use sc_telemetry::{TelemetryHandle};

// use slots::Slots;
use slot_worker::{
	// BackoffAuthoringBlocksStrategy, SlotInfo, StorageChanges,
	BackoffAuthoringBlocksStrategy, InherentDataProviderExt, StorageChanges,
	// SimpleSlotWorker,
	// ElectionWeightInfo,
};
pub use slot_worker::{SlotProportion, SlotResult};
pub use import_queue::{
	build_verifier, AuraVerifier, BuildVerifierParams, CheckForEquivocation,
	ImportQueueParams,
};

use schnorrkel::{
    keys::PublicKey,
    // vrf::{VRFOutput, VRFProof}
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

pub struct AuthorWorker<P, B, C, E, I, L, BS, SO, VL, CIDP>
where
	B: BlockT,
	P: Pair,
{
	pub client: Arc<C>,
	pub block_import: I,
	pub env: E,
	pub sync_oracle: SO,
	pub justification_sync_link: L,
	pub force_authoring: bool,
	pub backoff_authoring_blocks: Option<BS>,
	pub keystore: SyncCryptoStorePtr,
	pub block_proposal_slot_portion: SlotProportion,
	pub max_block_proposal_slot_portion: Option<SlotProportion>,
	pub telemetry: Option<TelemetryHandle>,

	pub create_inherent_data_providers: CIDP,
	// pub slots: Slots<B, SC, CIDP>,

	pub vote_link: VL,
	pub state_info: Option<StateInfo<B, P>>,
}

// impl<B, C, P, VL> AuthorWorker<B, C, P, VL>
// where
// 	// A: Codec + Debug,
// 	P: Pair + Send + Sync,
// 	P::Public: AppPublic + Encode + Decode + Debug,
// 	P::Signature: Encode + Decode,
// 	// A: AuthorityId<P>,
// 	B: BlockT,
// 	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
// 	C::Api: VoteApi<B, AuthorityId<P>>,
// 	VL: VoteLink<B> + Send + Clone,

// #[async_trait::async_trait]
impl<P, B, C, E, I, L, BS, SO, VL, CIDP, Error> AuthorWorker<P, B, C, E, I, L, BS, SO, VL, CIDP> 
where
	B: BlockT,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: TryFrom<Vec<u8>> + Encode + Decode,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + BlockOf + Sync + Send + 'static, 
	C::Api: VoteApi<B, AuthorityId<P>>,
	// E: Environment<B, Error = Error>,
	E: Environment<B, Error = Error> + Send + Sync,
	E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	VL: VoteLink<B> + Send + Clone,

	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	// PF: Environment<B, Error = Error> + Send + Sync + 'static,
	// PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	// SC: SelectChain<B>,
	L: sc_consensus::JustificationSyncLink<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
{
    // pub fn new(client: Arc<C>, keystore: SyncCryptoStorePtr, vote_link: VL)->Self{
	// 	Self{
	// 		client: client.clone(),
	// 		keystore,
	// 		vote_link: vote_link.clone(),
	// 		state_info: None,
	// 	}
    // }

	// type Claim = (PreDigest, P::Public);
	// type EpochData = Vec<AuthorityId<P>>;

	pub fn reset_state_info(&mut self, cur_header: &B::Header)->Result<(), String>{
		// basic state_info
		let committee_vec = authorities(self.client.as_ref(), &BlockId::Hash(cur_header.hash()))
			.map_err(|e|format!("get pallet authorities failed: {}", e))?;

		let (min_weight, max_weight) = 
			utils::caculate_min_max_election_weight(committee_vec.len(), MAX_VOTE_RANK);

		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, sp_application_crypto::key_types::AURA);

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

    fn generate_vrf_and_propagate(&mut self, cur_header: &B::Header)->Result<(u128, VRFSignature), String>{
    // fn propagate_vote_and_get_vrf(&mut self, cur_header: &B::Header)->Result<(u128, VRFSignature), String>{
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
				// log::warn!("Author, generate vrf failed: {}", e);
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

	fn generate_vrf_data(&self, cur_hash: &B::Hash)->Result<(u128, VRFSignature), String>{
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA
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

		let cur_weight = utils::caculate_weight_from_elections_with_detail(
			&state_info.pub_bytes,
			&state_info.election_vec,
			state_info.committee_vec.len(),
			MAX_VOTE_RANK
		);
		state_info.cur_weight = cur_weight;
		Ok(cur_weight)
    }

	fn get_min_max_weight(&self, header: &B::Header)->Result<(u64, u64), String>{
		let state_info = self.state_info.as_ref().ok_or(format!("no state info"))?;
		if header.hash() == state_info.cur_header.hash(){
			Ok((state_info.min_weight, state_info.max_weight))
		}
		else{
			Err(format!("invalid header"))
		}
	}

	fn verify_election(&self, election_data: &ElectionData<B>)->Result<(), String>{
		let state_info = self.state_info.as_ref().ok_or(format!("no state info"))?;

		// verify block_hash 
		if state_info.cur_header.hash() != election_data.block_hash{
			// log::info!("verify_election() failed, hash not eq, cur: {}, recv: {}", cur_hash, election_data.hash);
			Err(format!(
				"Invalid election: expect: #{}({}), recv: {}",
				state_info.cur_header.number(),
				state_info.cur_header.hash(),
				election_data.block_hash,
			))?;
		}

		// verify committee member
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

		// check signature
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

	// async fn proposal_block_v1(&mut self, vrf_sig: VRFSignature)->Result<(), String>{
	// 	let state_info = self.state_info.as_ref().cloned().ok_or(format!("no state info"))?;
	// 	if state_info.cur_weight < state_info.max_weight{
	// 		log::info!(
	// 			"Author.S1: timeout, prepare block at: #{} ({})",
	// 			state_info.cur_header.number(),
	// 			state_info.cur_header.hash(),
	// 		);
	// 		if let Ok(slot_info) = self.slots.default_slot().await{
	// 			let _ = self.produce_block(
	// 				slot_info,
	// 				&state_info.cur_header,
	// 				&vrf_sig,
	// 				// &state_info.vrf_signature,
	// 				state_info.election_vec.clone(),
	// 			).await;
	// 		}
	// 		// log::info!("produce block");
	// 	}
	// 	else{
	// 		log::info!(
	// 			"Author.S1: timeout, no weight prepare block at: #{} ({})",
	// 			state_info.cur_header.number(),
	// 			state_info.cur_header.hash(),
	// 		);
	// 	}
	// 	Ok(())
	// }

	fn claim(
		&mut self,
		vrf_sig: &VRFSignature,
		election_vec: Vec<ElectionData<B>>,
	) -> Option<(PreDigest, P::Public)> {
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA
		);

		if sr25519_public_keys.len() == 1{

			let pub_bytes = sr25519_public_keys[0].to_raw_vec();
			if let Ok(author) = <AuthorityId<P> as Decode>::decode(&mut pub_bytes.as_slice()){
				let pre_digest = PreDigest{
					// slot,
					// rand_bytes: rand_bytes,
					pub_key_bytes: author.to_raw_vec(),
					vrf_output_bytes: vrf_sig.output.to_bytes().encode(),
					vrf_proof_bytes: vrf_sig.proof.to_bytes().encode(),
					election_bytes: election_vec.encode()
				};
				return Some((pre_digest, author.clone()));
				// return Some((pre_digest, author.clone()));
			}
		}

		None
	}

	fn pre_digest_data(
		&self,
		claim: &(PreDigest, P::Public),
	) -> Vec<sp_runtime::DigestItem<B::Hash>> {
		// vec![<DigestItemFor<B> as CompatibleDigestItem<P::Signature>>::aura_pre_digest(slot.clone())]
		vec![<DigestItem<B::Hash> as CompatibleDigestItem<P::Signature>>::aura_pre_digest(claim.0.clone())]
	}

	fn epoch_data(
		&self,
		header: &B::Header,
	) -> Result<Vec<AuthorityId<P>>, sp_consensus::Error> {
		authorities(self.client.as_ref(), &BlockId::Hash(header.hash()))
	}

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
				<DigestItem<B::Hash> as CompatibleDigestItem<P::Signature>>::aura_seal(signature);

			let mut import_block = BlockImportParams::new(BlockOrigin::Own, header);
			import_block.post_digests.push(signature_digest_item);
			import_block.body = Some(body);
			import_block.state_action =
				StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
			import_block.fork_choice = Some(ForkChoiceStrategy::LongestChain);

			Ok(import_block)
		})
	}

	fn proposer(&mut self, header: &B::Header) -> 
		Pin<Box<dyn Future<Output = Result<E::Proposer, sp_consensus::Error>> + Send + 'static>>
	{
		Box::pin(
			self.env
				.init(header)
				.map_err(|e| sp_consensus::Error::ClientImport(format!("{:?}", e)).into()),
		)
	}

	async fn proposal_block(&mut self, vrf_sig: VRFSignature)->Result<(), String>{
		// let state_info = self.state_info.as_ref().ok_or(format!("no state info"))?;
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

			let epoch_data = match self.epoch_data(&parent_header) {
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

			// logs
			// let claim = self.claim_slot(slot_info.slot, vrf_sig, election_vec)?;
			// let logs = self.pre_digest_data(slot, &claim);
			let authorities_len = epoch_data.len();

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


			let (block, storage_proof) = (proposal.block, proposal.proof);
			let (header, body) = block.deconstruct();
			let header_num = *header.number();
			let header_hash = header.hash();
			let parent_hash = *header.parent_hash();

			let block_import_params_maker = self.block_import_params();
			let block_import_params = match block_import_params_maker(
				header,
				&header_hash,
				body.clone(),
				proposal.storage_changes,
				claim,
				epoch_data,
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

			// self.produce_block(
			// 	slot_info,
			// 	&state_info.cur_header,
			// 	&vrf_sig,
			// 	// &state_info.vrf_signature,
			// 	state_info.election_vec.clone(),
			// ).await.map_err(|e|format!("produce block failed: {}", e))?;
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

// impl<B, C, P, VL> slot_worker::SimpleSlotWorker<B> for AuthorWorker<B, C, P, VL>
// where
// 	// A: Codec + Debug,
// 	P: Pair + Send + Sync,
// 	P::Public: AppPublic + Encode + Decode + Debug,
// 	P::Signature: Encode + Decode,
// 	// A: AuthorityId<P>,
// 	B: BlockT,
// 	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
// 	C::Api: VoteApi<B, AuthorityId<P>>,
// 	VL: VoteLink<B> + Send + Clone,

// impl<P, B, C, E, I, L, BS, SO, SC, VL, CIDP, Error> slot_worker::SimpleSlotWorker<B>
// 	for AuthorWorker<P, B, C, E, I, L, BS, SO, VL, CIDP, SC> 
// where
// 	P: Pair + Send + Sync,
// 	P::Public: AppPublic + Encode + Decode + Debug,
// 	P::Signature: TryFrom<Vec<u8>> + Encode + Decode,
// 	B: BlockT,
// 	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + BlockOf + Sync + Send + 'static, 
// 	C::Api: VoteApi<B, AuthorityId<P>>,
// 	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
// 	E: Environment<B, Error = Error>,
// 	E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
// 	L: sc_consensus::JustificationSyncLink<B>,
// 	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
// 	SO: SyncOracle<B> + Send + Sync + Clone,
// 	SC: SelectChain<B>,
// 	// PF: Environment<B, Error = Error> + Send + Sync + 'static,
// 	// PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
// 	VL: VoteLink<B> + Send + Clone,
// 	CIDP: CreateInherentDataProviders<B, ()> + Send,
// 	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
// 	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
// {
// 	type BlockImport = I;
// 	type SyncOracle = SO;
// 	type JustificationSyncLink = L;
// 	type CreateProposer =
// 		Pin<Box<dyn Future<Output = Result<E::Proposer, sp_consensus::Error>> + Send + 'static>>;
// 	type Proposer = E::Proposer;
// 	type Claim = (PreDigest, P::Public);
// 	// type Claim = P::Public;
// 	type EpochData = Vec<AuthorityId<P>>;

// 	fn logging_target(&self) -> &'static str {
// 		"aura"
// 	}

// 	fn block_import(&mut self) -> &mut Self::BlockImport {
// 		&mut self.block_import
// 	}

// 	// fn block_notification_stream(&self)->ImportNotifications<B>{
// 	// 	self.client.import_notification_stream()
// 	// }

// 	fn epoch_data(
// 		&self,
// 		header: &B::Header,
// 		_slot: Slot,
// 	) -> Result<Self::EpochData, sp_consensus::Error> {
// 		authorities(self.client.as_ref(), &BlockId::Hash(header.hash()))
// 	}

// 	fn authorities_len(&self, epoch_data: &Self::EpochData) -> Option<usize> {
// 		Some(epoch_data.len())
// 	}

// 	// fn claim_slot(
// 	// 	&mut self,
// 	// 	_header: &B::Header,
// 	// 	slot: Slot,
// 	// 	_epoch_data: &Self::EpochData,
// 	// ) -> Option<Self::Claim> {
// 	// 	let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
// 	// 		&*self.keystore, 
// 	// 		sp_application_crypto::key_types::AURA
// 	// 	);

// 	// 	if sr25519_public_keys.len() > 0 {

// 	// 		let pub_bytes = sr25519_public_keys[0].to_raw_vec();
// 	// 		if let Ok(author) = <AuthorityId<P> as Decode>::decode(&mut pub_bytes.as_slice()){
// 	// 			// let pre_digest = PreDigest{slot: slot, public: author.to_raw_vec()};
// 	// 			let pre_digest = PreDigest{
// 	// 				slot,
// 	// 				rand_bytes: vec![],
// 	// 				pub_bytes: author.to_raw_vec(),
// 	// 				election_bytes: vec![],
// 	// 			};
// 	// 			return Some((pre_digest, author.clone()));
// 	// 		}
// 	// 	}

// 	// 	None
// 	// }

// 	fn claim_slot(
// 		&mut self,
// 		slot: Slot,
// 		vrf_sig: &VRFSignature,
// 		election_vec: Vec<ElectionData<B>>,
// 	) -> Option<Self::Claim> {
// 		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
// 			&*self.keystore, 
// 			sp_application_crypto::key_types::AURA
// 		);

// 		if sr25519_public_keys.len() == 1{

// 			let pub_bytes = sr25519_public_keys[0].to_raw_vec();
// 			if let Ok(author) = <AuthorityId<P> as Decode>::decode(&mut pub_bytes.as_slice()){
// 				let pre_digest = PreDigest{
// 					slot,
// 					// rand_bytes: rand_bytes,
// 					pub_key_bytes: author.to_raw_vec(),
// 					vrf_output_bytes: vrf_sig.output.to_bytes().encode(),
// 					vrf_proof_bytes: vrf_sig.proof.to_bytes().encode(),
// 					election_bytes: election_vec.encode()
// 				};
// 				return Some((pre_digest, author.clone()));
// 				// return Some((pre_digest, author.clone()));
// 			}
// 		}

// 		None
// 	}

// 	// fn is_committee(&mut self, hash: &B::Hash)->bool{
// 	// 	let committee = match authorities(self.client.as_ref(), &BlockId::Hash(hash.clone())){
// 	// 		Ok(x)=>x,
// 	// 		Err(_)=> return false
// 	// 	};

// 	// 	for author in committee.iter(){
// 	// 		if SyncCryptoStore::has_keys(
// 	// 			&*self.keystore,
// 	// 			&[(author.to_raw_vec(), sp_application_crypto::key_types::AURA)],
// 	// 		){
// 	// 			return true;
// 	// 		}
// 	// 	}
// 	// 	return false;
// 	// }

// 	// add by user
// 	fn notify_slot(&self, _header: &B::Header, _slot: Slot, _epoch_data: &Self::EpochData) {
// 	}

// 	fn pre_digest_data(
// 		&self,
// 		_slot: Slot,
// 		claim: &Self::Claim,
// 	) -> Vec<sp_runtime::DigestItem> {
// 		// vec![<DigestItemFor<B> as CompatibleDigestItem<P::Signature>>::aura_pre_digest(slot.clone())]
// 		vec![<DigestItem as CompatibleDigestItem<P::Signature>>::aura_pre_digest(claim.0.clone())]
// 	}

// 	fn block_import_params(
// 		&self,
// 	) -> Box<
// 		dyn Fn(
// 				B::Header,
// 				&B::Hash,
// 				Vec<B::Extrinsic>,
// 				StorageChanges<sp_api::TransactionFor<C, B>, B>,
// 				Self::Claim,
// 				Self::EpochData,
// 			) -> Result<
// 				sc_consensus::BlockImportParams<B, sp_api::TransactionFor<C, B>>,
// 				sp_consensus::Error,
// 			> + Send
// 			+ 'static,
// 	> {
// 		let keystore = self.keystore.clone();
// 		Box::new(move |header, header_hash, body, storage_changes, (_, public), _epoch| {
// 			// sign the pre-sealed hash of the block and then
// 			// add it to a digest item.
// 			let public_type_pair = public.to_public_crypto_pair();
// 			let public = public.to_raw_vec();
// 			let signature = SyncCryptoStore::sign_with(
// 				&*keystore,
// 				<AuthorityId<P> as AppKey>::ID,
// 				&public_type_pair,
// 				header_hash.as_ref(),
// 			)
// 			.map_err(|e| sp_consensus::Error::CannotSign(public.clone(), e.to_string()))?
// 			.ok_or_else(|| {
// 				sp_consensus::Error::CannotSign(
// 					public.clone(),
// 					"Could not find key in keystore.".into(),
// 				)
// 			})?;
// 			let signature = signature
// 				.clone()
// 				.try_into()
// 				.map_err(|_| sp_consensus::Error::InvalidSignature(signature, public))?;

// 			let signature_digest_item =
// 				<DigestItem as CompatibleDigestItem<P::Signature>>::aura_seal(signature);

// 			let mut import_block = BlockImportParams::new(BlockOrigin::Own, header);
// 			import_block.post_digests.push(signature_digest_item);
// 			import_block.body = Some(body);
// 			import_block.state_action =
// 				StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
// 			import_block.fork_choice = Some(ForkChoiceStrategy::LongestChain);

// 			Ok(import_block)
// 		})
// 	}

// 	fn force_authoring(&self) -> bool {
// 		self.force_authoring
// 	}

// 	fn should_backoff(&self, slot: Slot, chain_head: &B::Header) -> bool {
// 		if let Some(ref strategy) = self.backoff_authoring_blocks {
// 			if let Ok(pre_digest) = find_pre_digest::<B, P::Signature>(chain_head) {
// 				let chain_head_slot = pre_digest.slot;
// 				return strategy.should_backoff(
// 					*chain_head.number(),
// 					chain_head_slot,
// 					self.client.info().finalized_number,
// 					slot,
// 					self.logging_target(),
// 				)
// 			}
// 		}
// 		false
// 	}

// 	fn sync_oracle(&mut self) -> &mut Self::SyncOracle {
// 		&mut self.sync_oracle
// 	}

// 	// fn vote_link(&mut self) -> &mut Self::VoteLink{
// 	// 	&mut self.vote_link
// 	// }

// 	fn justification_sync_link(&mut self) -> &mut Self::JustificationSyncLink {
// 		&mut self.justification_sync_link
// 	}

// 	fn proposer(&mut self, block: &B::Header) -> Self::CreateProposer {
// 		Box::pin(
// 			self.env
// 				.init(block)
// 				.map_err(|e| sp_consensus::Error::ClientImport(format!("{:?}", e)).into()),
// 		)
// 	}

// 	fn telemetry(&self) -> Option<TelemetryHandle> {
// 		self.telemetry.clone()
// 	}

// 	fn proposing_remaining_duration(&self, slot_info: &SlotInfo<B>) -> std::time::Duration {
// 		let parent_slot = find_pre_digest::<B, P::Signature>(&slot_info.chain_head).ok().map(|d| d.slot);

// 		slot_worker::proposing_remaining_duration(
// 			parent_slot,
// 			slot_info,
// 			&self.block_proposal_slot_portion,
// 			self.max_block_proposal_slot_portion.as_ref(),
// 			slot_worker::SlotLenienceType::Exponential,
// 			self.logging_target(),
// 		)
// 	}
// }

// pub async fn run_author_worker<B, C, P, SC, SO, VL>(
// 	client: Arc<C>,
// 	mut worker: AuthorWorker<B, C, P, VL>,
// 	select_chain: SC,
// 	mut sync_oracle: SO,
// 	mut vote_link: VL,
// )
// where
// 	B: BlockT,
// 	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
// 	P: Pair + Send + Sync,
// 	P::Public: AppPublic + Encode + Decode + Debug,
// 	P::Signature: Encode + Decode,
// 	C::Api: VoteApi<B, AuthorityId<P>>,
// 	SC: SelectChain<B>,
// 	SO: SyncOracle<B> + Send,
// 	VL: VoteLink<B> + Send + Clone,

// fn reset_state_info()
// fn propagate_vote_and_get_vrf()
// fn get_min_max_weight()
// fn recv_election_and_update_weight()
// fn proposal_block()

pub async fn run_author_worker<P, B, C, I, L, BS, SO, SC, PF, VL, CIDP, Error>(
	client: Arc<C>,
	mut worker: AuthorWorker<P, B, C, PF, I, L, BS, SO, VL, CIDP>,
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
	// C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
	C::Api: VoteApi<B, AuthorityId<P>>,
	VL: VoteLink<B> + Send + Clone,
	// E: Environment<B, Error = Error>,
	// E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	SC: SelectChain<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	L: sc_consensus::JustificationSyncLink<B>,
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
                let mut delay = Delay::new(Duration::from_secs(10));
                let timeout = &mut delay;

                loop{
                    futures::select!{
                        // block = imported_blocks_stream.select_next_some()=>{
                        block = imported_blocks_stream.next()=>{
                            if let Some(block) = block{
                                log::info!("Author.S0, import block: #{} ({})", block.header.number(), block.hash);
                                if sync_oracle.is_major_syncing(){
                                    state = AuthorState::WaitStart;
                                    break;
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
            AuthorState::WaitProposal(cur_header)=>{
				log::info!(
					"► AuthorState::S1 #{} ({}), propagate vote and wait proposal",
					cur_header.number(),
					cur_header.hash(),
				);
				if let Err(e) = worker.reset_state_info(&cur_header){
					log::warn!("Author.S1: reset state info err, {}", e);
					state = AuthorState::WaitStart;
					continue 'outer;
				}

				let (_, vrf_sig) = match worker.generate_vrf_and_propagate(&cur_header){
					Ok(x)=>x,
					Err(e)=>{
						log::warn!("Author.S1: propagate vote err: {}", e);
						state = AuthorState::WaitStart;
						continue 'outer;
					}
				};

				// let worker = AuthorWorker::new(&cur_header);
				// worker.reset_state(&cur_header);
				let (min_weight, max_weight) = match worker.get_min_max_weight(&cur_header){
					Ok(x)=>x,
					Err(e)=>{
						log::warn!("Author.S1: get min max weight failed: {}", e);
						state = AuthorState::WaitStart;
						continue 'outer;
					}
				};

				let full_timeout_duration = Duration::from_secs(COMMITTEE_TIMEOUT*3);
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
                        // block = imported_blocks_stream.select_next_some()=>{
                        block = imported_blocks_stream.next()=>{
                            if let Some(block) = block{
                                log::info!("Author.S1, import block: #{} ({})", block.header.number(), block.hash);

								if sync_oracle.is_major_syncing(){
									state = AuthorState::WaitStart;
									continue 'outer;
								}

                                state = AuthorState::WaitProposal(block.header);
                                continue 'outer;
                            }
                        },
                        election = election_rx.select_next_some()=>{
							// log::info!("Author.S1, recv election");
							if let Ok(cur_weight) = worker.recv_election_and_update_weight(&election){
								rest_timeout_rate = update_s1_timeout_rate(
									cur_weight,
									min_weight,
									max_weight,
									&mut min_weight_delay_count,
									&mut not_min_weight_delay_count,
								);
								continue;
							}
                        },
                        _ = timeout.fuse()=>{
							log::info!("Author.S1, timeout");
							match worker.proposal_block(vrf_sig).await{
								Ok(_)=>{},
								Err(e)=>{
									log::info!("Author.S1, propsoal block failed: {}", e);
								}
							}
							// if cur_election_weight < max_election_weight {
							// 	log::info!(
							// 		"Author.S1: timeout, prepare block at: #{} ({})",
							// 		cur_header.number(),
							// 		cur_header.hash(),
							// 	);
							// 	if let Ok(slot_info) = slots.default_slot().await{
							// 		let _ = worker.produce_block(slot_info, &cur_header, &vrf_signature, election_vec).await;
							// 	}
							// }
							// else{
							// 	log::info!(
							// 		"Author.S1: timeout, no weight build block at: #{} ({})",
							// 		cur_header.number(),
							// 		cur_header.hash(),
							// 	);
							// }

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

	fn update_s1_timeout_rate(
		cur_election_weight: u64,
		min_election_weight: u64,
		max_election_weight: u64,
		min_weight_delay_count: &mut u64,
		not_min_weight_delay_count: &mut u64,
	)->f32
	{
		let rest_timeout_rate = {
			if cur_election_weight <= min_election_weight{
				if *min_weight_delay_count < 10 { 
					*min_weight_delay_count += 1;
					0.01 
				}
				else {
					0.0
				}
			}
			else{
				let rate = (cur_election_weight - min_election_weight) as f32 /
					(max_election_weight - min_election_weight) as f32;
				
				if *not_min_weight_delay_count < 20{
					*not_min_weight_delay_count += 1;
					rate + 0.1f32
				}
				else{
					rate
				}

				// 0.02f32.max(
				// 	(cur_election_weight - min_election_weight) as f32 /
				// 	(max_election_weight - min_election_weight) as f32
				// )
			}
		};

		return rest_timeout_rate;
	}
}
