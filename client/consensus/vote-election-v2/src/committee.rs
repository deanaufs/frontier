use crate::utils;
use crate::{
    AuthorityId, MAX_VOTE_RANK, ElectionWeightInfo, 
    authorities, find_pre_digest,
};

use codec::{Decode, Encode};
use futures::{channel::mpsc, prelude::*};
use futures_timer::Delay;
use std::{
    sync::Arc,
    fmt::Debug,
	marker::PhantomData,
    time::{Duration},
    collections::{BTreeMap, HashMap},
};

use sc_client_api::{
	BlockchainEvents, BlockOf, FinalityNotification,
	// BlockchainEvents, ImportNotifications, BlockOf, FinalityNotification,
};

use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
// use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr, vrf::VRFSignature};

use sp_core::crypto::{Pair, Public};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppKey, AppPublic, ByteArray};
use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT, Zero},
	// traits::{Block as BlockT, HashFor, Header as HeaderT, Zero},
};

use sp_consensus::{
    // Error as ConsensusError,
    SelectChain, SyncOracle, VELink as VoteLink,
    // CanAuthorWith, Proposer, SelectChain, SlotData, SyncOracle, VELink as VoteLink,
	VoteElectionRequest, VoteData, ElectionData
};

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	// inherents::{InherentDataProvider, InherentType as AuraInherent, INHERENT_IDENTIFIER},
	AuraApi as VoteApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};

use schnorrkel::{
    keys::PublicKey,
    vrf::{VRFOutput, VRFProof}
};

// type AuthorityId<P> = <P as Pair>::Public;

pub struct CommitteeWorker<B, C, P, VL>
where
    B: BlockT,
{
    client: Arc<C>,
	keystore: SyncCryptoStorePtr,
    root_vote_map: HashMap<B::Hash, BTreeMap<u128, VoteData<B>>>,
    vote_link: VL,
	_phantom: PhantomData<P>,
}

impl<B, C, P, VL> CommitteeWorker<B, C, P, VL>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockOf + Send + Sync + 'static,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: Encode + Decode,
	// P::Public: AppPublic + Hash + Member + Encode + Decode,
	C::Api: VoteApi<B, AuthorityId<P>>,
	VL: VoteLink<B> + Send,
{
    pub fn new(client: Arc<C>, keystore: SyncCryptoStorePtr, vote_link: VL)->Self{
        Self{
            client: client,
            keystore: keystore,
            root_vote_map: HashMap::new(),
            vote_link: vote_link,
            _phantom: PhantomData,
        }
    }

    fn on_finalize_block(&mut self, block: Option<FinalityNotification<B>>){
		block.map(|block|self.root_vote_map.remove(&block.hash));
    }

    fn on_recv_vote(&mut self, vote_data: &VoteData<B>)->Result<(), String>{
        // log::info!("Recv vote at : {}", vote_data.block_hash);
        let vrf_num = self.verify_vote(&vote_data)?;

        if let Some(bt_map) = self.root_vote_map.get_mut(&vote_data.block_hash){
			bt_map.insert(vrf_num, vote_data.clone());
			if bt_map.len() <= MAX_VOTE_RANK {
				return Ok(())
			}

			let mut keys = bt_map.keys().cloned().collect::<Vec<_>>();
			while bt_map.len() > MAX_VOTE_RANK{
				keys.pop().as_ref().map(|x|{
					bt_map.remove(x)
				});
			}
        }
        else{
            let mut bt_map = BTreeMap::new();
            bt_map.insert(vrf_num, vote_data.clone());
            self.root_vote_map.insert(vote_data.block_hash, bt_map);
        }

        Ok(())
    }

    fn propagate_election(&mut self, cur_header: &B::Header){
        if !self.is_committee_at(&cur_header.hash()){
            return
        }

        let mut election_result = vec![];
        let cur_hash = cur_header.hash();
        if let Some(bt_map) = self.root_vote_map.get(&cur_hash){
            for (_, (k, v)) in bt_map.iter().enumerate(){
                // log::info!("{}:{:?}", i, v);
                // log::info!("--Committee send back: ({:?}, {:?}) {}", v.sig_bytes[0..2], v.pub_bytes[0..2], cur_hash);
                // log::info!("--Committee send: ({:?}), {}", v.pub_bytes, cur_hash);
                log::info!("Committee.S1, pre send vrf: 0x{:0>32X}", k);
                election_result.push(v.clone());
            }

            // log::info!("--Committee: send back vote: {}", cur_hash);
            let _ = self.do_propagate_election(cur_hash, election_result);
        }
        else{
            log::warn!("Committee.S1: no vote for hash: #{}({})", cur_header.number(), cur_hash);
        }
        // self.ve_link.ve_request(VoteElectionRequest::PropagateElection(election));
        // log::info!("implment committee work propagate election");
    }

    fn is_committee_at(&self, hash: &B::Hash)->bool{
		let committee = match authorities(self.client.as_ref(), &BlockId::Hash(hash.clone())){
			Ok(x)=>x,
			Err(_)=> return false
		};

		for author in committee.iter(){
			if SyncCryptoStore::has_keys(
				&*self.keystore,
				&[(author.to_raw_vec(), sp_application_crypto::key_types::AURA)],
			){
				return true;
			}
		}
		return false;
    }

    fn verify_vote(&self, vote_data: &VoteData<B>)->Result<u128, String>{
		let transcript = make_transcript(&vote_data.block_hash.encode());
		let vrf_output = VRFOutput::from_bytes(vote_data.vrf_output_bytes.as_slice())
			.map_err(|e|format!("Decode vrf output failed: {}", e))?;

		let vrf_proof = VRFProof::from_bytes(vote_data.vrf_proof_bytes.as_slice())
			.map_err(|e|format!("Decode vrf proof failed: {}", e))?;

		schnorrkel::PublicKey::from_bytes(&vote_data.pub_bytes)
			.and_then(|p|{ p.vrf_verify(transcript, &vrf_output, &vrf_proof)})
			.and_then(|(inout, _)|{
				let vrf_num= u128::from_le_bytes(inout.make_bytes::<[u8; 16]>(VOTE_VRF_PREFIX));
				Ok(vrf_num) 
			}).map_err(|e|format!("Caculate vrf num failed: {}", e))
    }

    fn caculate_election_info_from_header(&self, header: &B::Header)->Result<ElectionWeightInfo, String>{
        if header.number().is_zero(){
            return Ok(ElectionWeightInfo{
                weight: u64::MAX,
                vrf_num: u128::MAX,
                exceed_half: true,
            });
        }

		let committee_list = authorities(self.client.as_ref(), &BlockId::Hash(header.hash()))
			.map_err(|e|format!("Get committee from pallet failed: {}", e))?;

		let pre_digest = find_pre_digest::<B, P::Signature>(&header)
			.map_err(|e|format!("find_pre_digest failed: {}", e))?;

		let vrf_output = VRFOutput::from_bytes(pre_digest.vrf_output_bytes.as_slice())
			.map_err(|e|format!("Decode vrf output failed: {}", e))?;

		let transcript = make_transcript(&header.parent_hash().encode());

		let public = PublicKey::from_bytes(pre_digest.pub_key_bytes.as_slice())
			.map_err(|e|format!("Decode public key failed: {}", e))?;

		let vrf_num = match vrf_output.attach_input_hash(&public, transcript){
			Ok(inout)=>u128::from_le_bytes(inout.make_bytes::<[u8; 16]>(VOTE_VRF_PREFIX)),
			Err(e)=> Err(format!("gen vrf number failed: {}", e))?,
		};

		let pub_bytes = pre_digest.pub_key_bytes;

		let election_bytes = pre_digest.election_bytes;
		let election_vec = <Vec<ElectionData<B>>>::decode(&mut election_bytes.as_slice())
			.map_err(|e|format!("Decode election data failed: {}", e))?;

		let committee_count = committee_list.len();

		let cur_election_weight = utils::caculate_weight_from_elections(&pub_bytes, &election_vec, committee_count, MAX_VOTE_RANK);
        let min_election_weight = utils::caculate_min_election_weight(committee_count, MAX_VOTE_RANK);
        let exceed_half = cur_election_weight <= min_election_weight;

		Ok(ElectionWeightInfo{
			weight: cur_election_weight,
			vrf_num,
            exceed_half,
		})
    }

	fn do_propagate_election(&mut self, block_hash: B::Hash, election_ret: Vec<VoteData<B>>)->Result<(), String>{
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA
		);

		if sr25519_public_keys.len() == 0{
			log::warn!("propagate_election failed");
		}

		let public_type_pair = sr25519_public_keys[0].to_public_crypto_pair();

		let mut pre_election:Vec<u8>= vec![];
		pre_election.extend(block_hash.encode().iter());
		pre_election.extend(election_ret.encode().iter());

		let msg = pre_election.as_slice();

		if let Some(sig_bytes) = SyncCryptoStore::sign_with(
			&*self.keystore,
			<AuthorityId<P> as AppKey>::ID,
			&public_type_pair,
			&msg,
		).map_err(|e|format!{"Sign election msg failed: {:?}", e})?
        {
			let pub_bytes = sr25519_public_keys[0].to_raw_vec();
			// let election_data = <ElectionData<B>>::new(hash, sig_bytes, election_ret, pub_bytes);
			let election_data = ElectionData::<B>{
				block_hash,
				sig_bytes,
				vote_list: election_ret,
				committee_pub_bytes: pub_bytes
			};
			self.vote_link.ve_request(VoteElectionRequest::PropagateElection(election_data));
		}
        Ok(())
	}
}

pub async fn start_committee_worker<B, C, P, SC, SO, VL>(
	client: Arc<C>,
	mut worker: CommitteeWorker<B, C, P, VL>,
    select_chain: SC,
	mut sync_oracle: SO,
	mut vote_link: VL,
)
where
    B: BlockT,
	// C: BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: Encode + Decode,
	C::Api: VoteApi<B, AuthorityId<P>>,
	SC: SelectChain<B>,
	SO: SyncOracle<B> + Send,
	VL: VoteLink<B> + Send,
{
    #[derive(Clone)]
    enum CommitteeState<H>{
        WaitStart,
        RecvVote(H),
    }

	let chain_head = match select_chain.best_chain().await{
		Ok(x) => x,
		Err(e) => {
			log::info!("fetch chain head err: {:?}", e);
			return
		}
	};

    let (mut init_state, mut genesis_header) = {
        if chain_head.number().is_zero(){
            (true, Some(chain_head))
        }
        else{
            (false, None)
        }
    };

	let (vote_tx, mut vote_rx) = mpsc::unbounded();
	vote_link.ve_request(VoteElectionRequest::BuildVoteStream(vote_tx));

	let mut imported_blocks_stream = client.import_notification_stream().fuse();
	let mut finality_notification_stream = client.finality_notification_stream().fuse();
    let mut state = CommitteeState::WaitStart;

    'outer: loop{
        match state {
            CommitteeState::WaitStart=>{
				log::info!("► CommitteeState::S0, wait start");
                let mut delay = Delay::new(Duration::from_secs(8));
                let timeout = &mut delay;

                loop{
                    futures::select!{
                        block = imported_blocks_stream.next()=>{
                            log::info!("Committee.S0, import block");
                            if let Some(block) = block{
                                init_state = false;
                                if sync_oracle.is_major_syncing(){
                                    continue;
                                }

                                if worker.is_committee_at(&block.hash){
                                    state = CommitteeState::RecvVote(block.header);
                                    continue 'outer;
                                }
                            }
                        },
                        _ = timeout.fuse()=>{
                            log::info!("Committee.S0, timeout");
                            if !init_state{
                                log::warn!("Committee.S0, timeout not from genesis");
                                state = CommitteeState::WaitStart;
                                continue 'outer;
                            }

                            init_state = false;

                            if let Some(header) = genesis_header.take(){
                                if worker.is_committee_at(&header.hash()){
                                    log::info!("Committee.S0, time out from genesis");
                                    state = CommitteeState::RecvVote(header);
                                    continue 'outer;
                                }
                            }
                        },
                        vote_data = vote_rx.select_next_some()=>{
                            // log::info!("Committee.S0, recv vote");
                            let _ = worker.on_recv_vote(&vote_data);
                            continue;
                        },
                        block = finality_notification_stream.next()=>{
                            // log::info!("Committee.S0, block finalize");
                            worker.on_finalize_block(block);
                            continue;
                        },
                    }
                }
            },

            CommitteeState::RecvVote(cur_header)=>{
				log::info!(
					"► CommitteeState::S1 #{} ({}), recv vote and send election",
					cur_header.number(),
					cur_header.hash(),
				);

                let cur_block_election_info = match worker.caculate_election_info_from_header(&cur_header){
                    Ok(v) => v,
                    Err(e) => {
						state = CommitteeState::WaitStart;
						log::warn!("Committee.S1, cacl block weight info error: {:?}", e);
						continue 'outer;
                    }
                };

                let mut delay = Delay::new(Duration::from_secs(8));
                let timeout = &mut delay;
                loop{
                    futures::select!{
                        // None = imported_blocks_stream.next()=>{},
                        block = imported_blocks_stream.next()=>{
                            log::info!("Committee.S1, import block");
                            // if let Some(block) = block{
                            //     state = CommitteeState::RecvVote(block.header);
                            //     continue 'outer;
                            // }

                            if let Some(block) = block{
                                if sync_oracle.is_major_syncing(){
                                    state = CommitteeState::WaitStart;
                                    continue 'outer;
                                }

                                if !worker.is_committee_at(&block.hash){
                                    continue;
                                }

                                if let Ok(import_block_election_info) = worker.caculate_election_info_from_header(&block.header){
                                    if import_block_election_info.exceed_half{
                                        log::info!(
                                            "Committee.S1: recv block with 50% exceed, #{}({})",
                                            block.header.number(),
                                            block.hash
                                        );
                                        state = CommitteeState::RecvVote(block.header);
                                        break;
                                    }

                                    if block.header.parent_hash() == cur_header.parent_hash() &&
                                        import_block_election_info.weight < cur_block_election_info.weight
                                    {
                                        log::info!("Committee.S1: recv block with higher priority, #{}({})", block.header.number(), block.hash);
                                        state = CommitteeState::RecvVote(block.header);
                                        break;
                                    }
                                }

                                log::warn!("Committee.S1: ignore this block: #{}({})", block.header.number(), block.header.hash());
                            }
                        },
                        _ = timeout.fuse()=>{
                            log::info!("Committee.S1, timeout");
                            worker.propagate_election(&cur_header);
                            state = CommitteeState::WaitStart;
                            continue 'outer;
                            // break;
                        },
                        vote_data = vote_rx.select_next_some()=>{
                            // log::info!("Committee.S1, recv vote");
                            let _ = worker.on_recv_vote(&vote_data);
                            continue;
                        },
                        block = finality_notification_stream.next()=>{
                            // log::info!("Committee.S1, block finalize");
                            worker.on_finalize_block(block);
                            continue;
                        },
                    }
                }
            },
        }
    }
}
