use crate::{Error, authorities, find_pre_digest};
use codec::{Codec, Decode};
use std::{
	fmt::Debug,
};
use sp_api::{ProvideRuntimeApi};
// use sp_api::HeaderT;

use sp_runtime::{
	traits::{Block as BlockT, Header},
};

use sc_client_api::{
	BlockOf,
	// backend::{AuxStore, Backend as ClientBackend, Finalizer},
	// BlockOf, UsageProvider, BlockchainEvents, ImportNotifications
};

use sp_runtime::{
    generic::BlockId,
};

use sp_consensus::{
	ElectionData,
    // Error as ConsensusError,
    // CanAuthorWith, Proposer, SelectChain, SlotData, SyncOracle, VELink as VoteLink,
	// VoteElectionRequest, VoteData, ElectionData
};

use sp_consensus_vote_election::VoteElectionApi;

pub fn caculate_block_weight<A, B, S, C>(
	header: &B::Header,
	client: &C,
	max_vote_rank: usize
)->Result<bool, Error<B>>
where
	A: Codec + Debug,
	B: BlockT,
	S: Codec,
	C: ProvideRuntimeApi<B> + BlockOf,
	C::Api: VoteElectionApi<B, A>,
{
	let committee_vec = authorities(client, &BlockId::Hash(header.hash()))
		.map_err(|_|Error::NoCommitteeFound)?;

	let committee_count = committee_vec.len();

	let pre_digest = find_pre_digest::<B, S>(header)
		.map_err(|_|Error::NoDigestFound)?;

	let pub_bytes = pre_digest.pub_key_bytes;

	let election_vec = <Vec<ElectionData<B>> as Decode>::decode(&mut pre_digest.election_bytes.as_slice())
		.map_err(|_|Error::ElectionDataDecodeFailed)?;
	
	let block_election_weight = caculate_weight_from_elections(
		&pub_bytes, &election_vec, committee_count, max_vote_rank);
	let min_election_weight = caculate_min_election_weight(committee_count, max_vote_rank);

	Ok(block_election_weight<=min_election_weight)
}

pub fn caculate_weight_from_elections<B: BlockT>(
	pub_bytes: &Vec<u8>,
	election_vec: &Vec<ElectionData<B>>,
	committee_count: usize,
	rank_count: usize,
)->u64{
	do_caculate_weight_from_elections(
		pub_bytes,
		election_vec,
		committee_count,
		rank_count,
		false,
	)
}

pub fn caculate_weight_from_elections_with_detail<B: BlockT>(
	pub_bytes: &Vec<u8>,
	election_vec: &Vec<ElectionData<B>>,
	committee_count: usize,
	rank_count: usize,
)->u64{
	do_caculate_weight_from_elections(
		pub_bytes,
		election_vec,
		committee_count,
		rank_count,
		true,
	)
}

fn do_caculate_weight_from_elections<B: BlockT>(
	pub_bytes: &Vec<u8>,
	election_vec: &Vec<ElectionData<B>>,
	committee_count: usize,
	rank_count: usize,
	show_detail: bool,
)->u64{
	let mut rank_vec = vec![];
	for election in election_vec.iter(){
		let rank = match election.vote_list.iter().position(|vote| vote.pub_bytes == *pub_bytes){
			Some(x) if x < rank_count =>x,
			// None => MAX_VOTE_RANK,
			_ => rank_count,
		};
		rank_vec.push(rank);
	}

	while rank_vec.len() < committee_count{
		rank_vec.push(rank_count);
	}

	rank_vec.sort();
	if show_detail{
		log::info!("{:?}, election result", rank_vec);
	}
	let weight = caculate_weight_from_ranks(&rank_vec, rank_count);
	weight
}

// fn caculate_election_weight_value(rank_vec: &Vec<usize>, max_vote_rank: usize)->u64{
fn caculate_weight_from_ranks(rank_vec: &Vec<usize>, max_vote_rank: usize)->u64{
	let mut ret = 0;
	let mut base = 1;
	let base_step = max_vote_rank+1;

	for rank in rank_vec.iter().rev(){
		ret += rank * base;
		base *= base_step;
	}
	ret as u64
}

pub fn caculate_min_max_election_weight(committee_count: usize, max_vote_rank: usize)->(u64, u64){
	let min_value = caculate_min_election_weight(committee_count, max_vote_rank);
	let max_value = caculate_max_election_weight(committee_count, max_vote_rank);
	(min_value, max_value)
}

pub fn caculate_min_election_weight(committee_count: usize, max_vote_rank: usize)->u64{
	let half_count = committee_count/2+1;
	let mut ret = 1;

	let mut i = 0;
	while i < (committee_count-half_count){
		ret *= (max_vote_rank+1) as u64;
		i += 1;
	}
	ret -1
}

pub fn caculate_max_election_weight(committee_count: usize, max_vote_rank: usize)->u64{
	let mut ret = 1;

	let mut i = 0;
	while i < committee_count{
		ret *= (max_vote_rank+1) as u64;
		i += 1;
	}
	ret-1
}
