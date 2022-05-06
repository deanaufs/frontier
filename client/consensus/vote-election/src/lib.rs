mod import_queue;
mod utils;
mod committee;
mod author;
mod slot_worker;
mod slots;
mod worker;

use codec::{Codec, Decode, Encode};
use futures::{prelude::*};
// use futures_timer::Delay;
use std::{
	convert::{TryFrom},
    sync::Arc,
    fmt::Debug,
	// marker::PhantomData,
    // time::{SystemTime, Duration},
    // collections::{BTreeMap, HashMap},
	// pin::Pin,
};

use sc_client_api::{
	BlockchainEvents, BlockOf,
	UsageProvider,
	backend::{AuxStore, Backend as ClientBackend, Finalizer},
	// BlockchainEvents, ImportNotifications, BlockOf, FinalityNotification,
};

use sp_keystore::{SyncCryptoStorePtr};

use sp_core::crypto::{Pair, };
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppPublic,};
use sp_blockchain::{HeaderBackend, Result as CResult};
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::{
    generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT, Zero, NumberFor},
	// traits::{Block as BlockT, HashFor, Header as HeaderT, Zero},
	// DigestItem,
};

use sp_consensus::{
    Error as ConsensusError,
    SelectChain, SyncOracle, VELink as VoteLink, SlotData,
    // CanAuthorWith, Proposer, SelectChain, SlotData, SyncOracle, VELink as VoteLink,
	Environment, Proposer, 
};

pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	Slot,
	// inherents::{InherentDataProvider, InherentType as AuraInherent, INHERENT_IDENTIFIER},
	AuraApi as VoteApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,
};
// use sp_consensus_slots::Slot;

use sc_consensus::{BlockImport, };
use sc_telemetry::{TelemetryHandle, };

use slots::Slots;
use slot_worker::{
	// BackoffAuthoringBlocksStrategy, SlotInfo, StorageChanges,
	BackoffAuthoringBlocksStrategy, InherentDataProviderExt,
	// SimpleSlotWorker,
	// ElectionWeightInfo,
};
pub use slot_worker::{SlotProportion, SlotResult};
pub use import_queue::{
	build_verifier, import_queue, AuraVerifier, BuildVerifierParams, CheckForEquivocation,
	ImportQueueParams,
};

// use schnorrkel::{
//     keys::PublicKey,
//     vrf::{VRFOutput, VRFProof}
// };
// use log::{debug, warn, info};


type AuthorityId<P> = <P as Pair>::Public;

pub type SlotDuration = slot_worker::SlotDuration<sp_consensus_vote_election::SlotDuration>;

pub const MAX_VOTE_RANK: usize = 5;
pub const COMMITTEE_TIMEOUT: u64 = 4;
pub const PROPOSAL_TIMEOUT: u64 = COMMITTEE_TIMEOUT - 1;

/// Get type of `SlotDuration` for Aura.
pub fn slot_duration<A, B, C>(client: &C) -> CResult<SlotDuration>
where
	A: Codec,
	B: BlockT,
	C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
	C::Api: VoteApi<B, A>,
{
	// SlotDuration::get_or_compute(client, |a, b| a.slot_duration(b).map_err(Into::into))
	let best_block_id = BlockId::Hash(client.usage_info().chain.best_hash);
	let slot_duration = client.runtime_api().slot_duration(&best_block_id)?;

	Ok(SlotDuration::new(slot_duration))
}

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
	// P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: Encode + Decode,
	// P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
	C::Api: VoteApi<B, AuthorityId<P>>,
	SC: SelectChain<B>,
	SO: SyncOracle<B> + Send,
	VL: VoteLink<B> + Send + Clone,
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

pub fn start_author<P, B, C, I, L, BS, CIDP, SO, SC, PF, VL, Error>(
	client: Arc<C>,
	block_import: I,
	proposal_factory: PF,
	create_inherent_data_providers: CIDP,
	sync_oracle: SO,
	justification_sync_link: L,
	force_authoring: bool,
	backoff_authoring_blocks: Option<BS>,
    keystore: SyncCryptoStorePtr,
	block_proposal_slot_portion: SlotProportion,
	max_block_proposal_slot_portion: Option<SlotProportion>,
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
	// C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + Sync + Send + 'static, 
	C::Api: VoteApi<B, AuthorityId<P>>,
	VL: VoteLink<B> + Send + Clone,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	// E: Environment<B, Error = Error>,
	// E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	SC: SelectChain<B>,
	L: sc_consensus::JustificationSyncLink<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
	// let slots =
	// 	Slots::new(slot_duration.slot_duration(), create_inherent_data_providers, select_chain.clone());

	let worker = author::AuthorWorker::<P, _, _, _, _, _, _, _, _, _>{
		client: client.clone(),
		block_import,
		env: proposal_factory,
		sync_oracle: sync_oracle.clone(),
		justification_sync_link,
		force_authoring,
		backoff_authoring_blocks,
		keystore,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
		telemetry,
		create_inherent_data_providers,
		// slots,
		vote_link: vote_link.clone(),
		state_info: None,
	};

	Ok(author::run_author_worker::<P, B, C, I, L, BS, SO, SC, PF, VL, CIDP, Error>(
        client,
        worker,
        select_chain,
        sync_oracle,
        vote_link,
	))
}

pub async fn run_simple_finalizer<A, B, C, CB, P>(client: Arc<C>)
where
    A: Codec + Debug,
    B: BlockT,
	CB: ClientBackend<B>,
    C: BlockchainEvents<B> + Finalizer<B, CB> + ProvideRuntimeApi<B> + BlockOf + Sync,
	C::Api: VoteApi<B, A>,
	P: Pair + Send + Sync,
	// P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
	P::Signature: TryFrom<Vec<u8>> + Encode + Decode + Debug,
{
	let mut imported_blocks_stream = client.import_notification_stream();
	let mut pre_finalize_vec = vec![];

    loop{
        if let Some(block)= imported_blocks_stream.next().await{

			if let Ok(can_finalize) = utils::caculate_block_weight::<A, B, P::Signature, C>(
				&block.header, client.as_ref(), MAX_VOTE_RANK,
			){
				if !can_finalize{
					continue;
				}

				pre_finalize_vec.push(block.hash);
				// log::info!(
				// 	"⇩ Finalizer: buffer finalize block({}): #{} ({})",
				// 	pre_finalize_vec.len(),
				// 	block.header.number(),
				// 	block.hash
				// );
				while pre_finalize_vec.len() > 3{
					let finalize_hash = pre_finalize_vec.remove(0);

					match client.finalize_block(BlockId::Hash(finalize_hash), None, true){
						Err(e) => {
							log::warn!("Failed to finalize block {:?}", e);
							// rpc::send_result(&mut sender, Err(e.into()))
						},
						Ok(()) => {
							log::info!("✅ Successfully finalized block: {}", block.hash);
							// rpc::send_result(&mut sender, Ok(()))
						},
					}
				}

				// min_election_weight: authority_len, MAX_VOTE_RANK
				// if let Ok(committee_vec) = authorities(client.as_ref(), &BlockId::Hash(block.hash)){
				//     let min_election_weight = utils::caculate_min_election_weight(committee_vec.len(), MAX_VOTE_RANK);

				// 	if let Ok(weight) = utils::caculate_block_weight::<A, B, P::Signature, C>(&block.header, client.as_ref()){

				// 		if weight <= min_election_weight{
				// 			pre_finalize_vec.push(block.hash);
				// 			// log::info!(
				// 			// 	"⇩ Finalizer: buffer finalize block({}): #{} ({})",
				// 			// 	pre_finalize_vec.len(),
				// 			// 	block.header.number(),
				// 			// 	block.hash
				// 			// );
				// 			while pre_finalize_vec.len() > 3{
				// 				let finalize_hash = pre_finalize_vec.remove(0);

				// 				match client.finalize_block(BlockId::Hash(finalize_hash), None, true){
				// 					Err(e) => {
				// 						log::warn!("Failed to finalize block {:?}", e);
				// 						// rpc::send_result(&mut sender, Err(e.into()))
				// 					},
				// 					Ok(()) => {
				// 						log::info!("✅ Successfully finalized block: {}", block.hash);
				// 						// rpc::send_result(&mut sender, Ok(()))
				// 					},
				// 				}
				// 			}

				// 		}
				// 	}
				// }
			}
		}
    }
}

pub fn authorities<A, B, C>(client: &C, at: &BlockId<B>) -> Result<Vec<A>, ConsensusError>
where
	A: Codec + Debug,
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockOf,
	C::Api: VoteApi<B, A>,
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
			// authority_index: 0u32,
			// slot: 0.into(), 
			pub_key_bytes: vec![],
			vrf_output_bytes: vec![],
			vrf_proof_bytes: vec![],
			election_bytes: vec![],
		})
	}

	let mut pre_digest: Option<_> = None;
	for log in header.digest().logs() {
		log::trace!(target: "vote", "Checking log {:?}", log);
		match (CompatibleDigestItem::<Signature>::as_aura_pre_digest(log), pre_digest.is_some()) {
		// match (log.as_aura_pre_digest(), pre_digest.is_some()){
			(Some(_), true) => return Err(vote_err(Error::MultipleHeaders)),
			(None, _) => log::trace!(target: "vote", "Ignoring digest not meant for us"),
			(s, false) => pre_digest = s,
		}
	}
	pre_digest.ok_or_else(|| vote_err(Error::NoDigestFound))
}