use codec::{Codec, Encode, Decode};

use std::{
	convert::TryFrom,
    fmt::Debug,
    sync::Arc,
    hash::Hash,
};
use futures::prelude::*;

use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT, Member, One},
};

use sc_client_api::{
	BlockchainEvents, BlockOf,
	backend::{Backend as ClientBackend, Finalizer},
};
use sp_api::ProvideRuntimeApi;
use sp_core::crypto::Pair;

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	Slot,
	VoteElectionApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};

use crate::MAX_VOTE_RANK;
use crate::utils;

pub async fn run_simple_finalizer<A, B, C, CB, P>(client: Arc<C>)
where
    A: Codec + Debug,
    B: BlockT,
	CB: ClientBackend<B>,
    C: BlockchainEvents<B> + Finalizer<B, CB> + ProvideRuntimeApi<B> + BlockOf + Sync,
	C::Api: VoteElectionApi<B, A>,
	P: Pair + Send + Sync,
	P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
{
	let mut imported_blocks_stream = client.import_notification_stream();
	let mut pre_finalize_vec = vec![];

    loop{
        if let Some(block)= imported_blocks_stream.next().await{

            // avoid re-finalize block #0
			if block.header.number().is_one() {
				continue;
			}

            if let Ok(can_finalize) = utils::caculate_block_weight::<A, B, P::Signature, C>(&block.header, client.as_ref(), MAX_VOTE_RANK){

                if can_finalize{

                    pre_finalize_vec.push(block.header.clone());

                    while pre_finalize_vec.len() > 2{
                        let finalize_header = pre_finalize_vec.remove(0);

                        match client.finalize_block(BlockId::Hash(finalize_header.hash()), None, true){
                            Ok(()) => {
                                log::info!("✅ Successfully finalized block: #{} ({})", finalize_header.number(), finalize_header.hash());
                            },
                            Err(e) => {
                                log::warn!("Failed to finalize block #{} ({}) {:?}", finalize_header.number(), finalize_header.hash(), e);
                            },
                        }
                    }

                }
            }
        }
    }
}

