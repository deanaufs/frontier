mod import_queue;
mod utils;
mod committee;
mod author;
mod finalizer;
mod worker;

use codec::{Codec, Decode, Encode};
use futures::{prelude::*};
use std::{
	convert::{TryFrom},
    sync::Arc,
    fmt::Debug,
};

use sc_client_api::{
	BlockchainEvents, BlockOf,
};

use sp_keystore::{SyncCryptoStorePtr};

use sp_core::crypto::{Pair, };
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppPublic,};
use sp_blockchain::{HeaderBackend, /*Result as CResult*/};
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT, Zero},
};

use sp_consensus::{
    Error as ConsensusError,
    SelectChain, SyncOracle, VELink ,
	Environment, Proposer, 
};

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	Slot,
	VoteElectionApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};

use sc_consensus::{BlockImport, };
use sc_telemetry::{TelemetryHandle, };

use worker::InherentDataProviderExt;
pub use import_queue::{
	build_verifier, import_queue, AuraVerifier, BuildVerifierParams, CheckForEquivocation,
	ImportQueueParams,
};

pub use finalizer::run_simple_finalizer;

type AuthorityId<P> = <P as Pair>::Public;

pub const MAX_VOTE_RANK: usize = 5;

pub const COMMITTEE_TIMEOUT: u64 = 4;
pub const COMMITTEE_S0_TIMEOUT: u64 = COMMITTEE_TIMEOUT + 2;

pub const AUTHOR_S0_TIMEOUT: u64 = COMMITTEE_S0_TIMEOUT - 1;
pub const AUTHOR_S1_TIMEOUT: u64 = COMMITTEE_TIMEOUT * 3;

pub const PROPOSAL_TIMEOUT: u64 = COMMITTEE_TIMEOUT - 1;


pub fn start_committee<P, B, C, SC, SO, VL>(
    client: Arc<C>,
    keystore: SyncCryptoStorePtr,
    select_chain: SC,
    sync_oracle: SO,
    vote_link: VL,
) -> Result<impl Future<Output = ()>, sp_consensus::Error>
where
    B: BlockT,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
	P: Pair + Send + Sync,
	P::Public: AppPublic + Encode + Decode + Debug,
	P::Signature: Encode + Decode,
	C::Api: VoteElectionApi<B, AuthorityId<P>>,
	SC: SelectChain<B>,
	SO: SyncOracle<B> + Send,
	VL: VELink<B> + Send + Clone,
{
    let worker = committee::CommitteeWorker::new(client.clone(), keystore, vote_link.clone());

    Ok(committee::start_committee_worker::<B, C, P, SC, SO, VL>(
        client,
        worker,
        select_chain,
        sync_oracle,
        vote_link,
    ))
}

pub fn start_author<P, B, C, I, CIDP, SO, SC, PF, VL, Error>(
	client: Arc<C>,
	block_import: I,
	proposal_factory: PF,
	create_inherent_data_providers: CIDP,
	sync_oracle: SO,
	force_authoring: bool,
    keystore: SyncCryptoStorePtr,
	telemetry: Option<TelemetryHandle>,
	select_chain: SC,
	vote_link: VL,
)->Result<impl Future<Output = ()>, sp_consensus::Error>
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
	let worker = author::AuthorWorker::<P, _, _, _, _, _, _, _>{
		client: client.clone(),
		block_import,
		env: proposal_factory,
		sync_oracle: sync_oracle.clone(),
		force_authoring,
		keystore,
		telemetry,
		create_inherent_data_providers,
		vote_link: vote_link.clone(),
		state_info: None,
	};

	Ok(author::run_author_worker::<P, B, C, I, SO, SC, PF, VL, CIDP, Error>(
        client,
        worker,
        select_chain,
        sync_oracle,
        vote_link,
	))
}

pub fn authorities<A, B, C>(client: &C, at: &BlockId<B>) -> Result<Vec<A>, ConsensusError>
where
	A: Codec + Debug,
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockOf,
	C::Api: VoteElectionApi<B, A>,
{
	client
		.runtime_api()
		.authorities(at)
		.ok()
		.ok_or_else(|| sp_consensus::Error::InvalidAuthoritiesSet.into())
}

struct ElectionWeightInfo{
	pub weight: u64,
	pub vrf_num: u128,
    pub exceed_half: bool,
}

pub fn vote_err<B: BlockT>(error: Error<B>) -> Error<B> {
	log::debug!(target: "vote", "{}", error);
	error
}

#[derive(derive_more::Display, Debug)]
pub enum Error<B: BlockT> {
	#[display(fmt = "Multiple Aura pre-runtime headers")]
	MultipleHeaders,
	#[display(fmt = "No Aura pre-runtime digest found")]
	NoDigestFound,
	#[display(fmt = "No election data found in digest")]
	NoElectionDataFound,
	#[display(fmt = "Election data decode failed")]
	ElectionDataDecodeFailed,
	#[display(fmt = "get committee member failed")]
	NoCommitteeFound,
	#[display(fmt = "decode vrfoutput failed")]
	VRFOutputDecodeFailed,
	#[display(fmt = "decode vrfproof failed")]
	VRFProofDecodeFailed,
	#[display(fmt = "vrf verify failed")]
	VRFVerifyFailed,
	#[display(fmt = "bad predigest data")]
	BadPredigest,
	#[display(fmt = "bad election signature bytes")]
	BadElectionSignatureBytes,
	#[display(fmt = "bad election committee bytes")]
	BadElectionCommitteeBytes,

	#[display(fmt = "Header {:?} is unsealed", _0)]
	HeaderUnsealed(B::Hash),
	#[display(fmt = "Header {:?} has a bad seal", _0)]
	HeaderBadSeal(B::Hash),
	// #[display(fmt = "Slot Author not found")]
	// SlotAuthorNotFound,
	#[display(fmt = "Bad signature on {:?}", _0)]
	BadSignature(B::Hash),
	#[display(fmt = "Election hash not match: {:?}", _0)]
	BadElection(B::Hash),
	Client(sp_blockchain::Error),
	#[display(fmt = "Unknown inherent error for identifier: {}", "String::from_utf8_lossy(_0)")]
	UnknownInherentError(sp_inherents::InherentIdentifier),
	#[display(fmt = "Inherent error: {}", _0)]
	Inherent(sp_inherents::Error),
}

impl<B: BlockT> std::convert::From<Error<B>> for String {
	fn from(error: Error<B>) -> String {
		error.to_string()
	}
}

pub fn find_pre_digest<B: BlockT, Signature: Codec>(header: &B::Header) -> Result<PreDigest, Error<B>> {
	if header.number().is_zero() {
		return Ok(PreDigest{
			pub_key_bytes: vec![],
			vrf_output_bytes: vec![],
			vrf_proof_bytes: vec![],
			election_bytes: vec![],
		})
	}

	let mut pre_digest: Option<_> = None;
	for log in header.digest().logs() {
		log::trace!(target: "vote", "Checking log {:?}", log);
		match (CompatibleDigestItem::<Signature>::as_ve_pre_digest(log), pre_digest.is_some()) {
			(Some(_), true) => return Err(vote_err(Error::MultipleHeaders)),
			(None, _) => log::trace!(target: "vote", "Ignoring digest not meant for us"),
			(s, false) => pre_digest = s,
		}
	}
	pre_digest.ok_or_else(|| vote_err(Error::NoDigestFound))
}