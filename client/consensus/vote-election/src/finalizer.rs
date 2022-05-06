use codec::{Codec, Encode, Decode};

use std::{
	convert::TryFrom,
    fmt::Debug,
    sync::Arc,
};

use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT},
};

use sc_client_api::{
	BlockchainEvents, BlockOf,
	backend::{Backend as ClientBackend, Finalizer},
	// BlockchainEvents, ImportNotifications, BlockOf, FinalityNotification,
};
use sp_api::ProvideRuntimeApi;
use sp_core::crypto::Pair;

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	Slot,
	// inherents::{InherentDataProvider, InherentType as AuraInherent, INHERENT_IDENTIFIER},
	AuraApi as VoteApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};

use crate::MAX_VOTE_RANK;
use crate::authorities;
use crate::utils;

pub async fn run_simple_finalizer<A, B, C, CB, P>(client: Arc<C>)
where
    A: Codec + Debug,
    B: BlockT,
	CB: ClientBackend<B>,
    C: BlockchainEvents<B> + Finalizer<B, CB> + ProvideRuntimeApi<B> + BlockOf + Sync,
	C::Api: VoteApi<B, A>,
	P: Pair + Send + Sync,
	P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
{
	let mut imported_blocks_stream = client.import_notification_stream();
	let mut pre_finalize_vec = vec![];

    loop{
        if let Some(block)= imported_blocks_stream.next().await{
			if block.header.number().is_one() {
				// avoid re-finalize block #0
				continue;
			}

            // min_election_weight: authority_len, MAX_VOTE_RANK
            if let Ok(committee_vec) = authorities(client.as_ref(), &BlockId::Hash(block.hash)){
                let min_election_weight = utils::caculate_min_election_weight(committee_vec.len(), MAX_VOTE_RANK);

				if let Ok(weight) = utils::caculate_block_weight::<A, B, P::Signature, C>(&block.header, client.as_ref(), MAX_VOTE_RANK){

					if weight <= min_election_weight{
						pre_finalize_vec.push(block.header.parent_hash().clone());
						// pre_finalize_vec.push(block.hash);
						while pre_finalize_vec.len() > 2{
							let finalize_hash = pre_finalize_vec.remove(0);

							match client.finalize_block(BlockId::Hash(finalize_hash.clone()), None, true){
								Ok(()) => {
									log::info!("✅ Successfully finalized block: {}", finalize_hash);
									// rpc::send_result(&mut sender, Ok(()))
								},
								Err(e) => {
									log::warn!("Failed to finalize block {:?}", e);
									// rpc::send_result(&mut sender, Err(e.into()))
								},
							}
						}

					}
				}
            }
        }
    }
}

