// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Aura (Authority-round) consensus in substrate.
//!
//! Aura works by having a list of authorities A who are expected to roughly
//! agree on the current time. Time is divided up into discrete slots of t
//! seconds each. For each slot s, the author of that slot is A[s % |A|].
//!
//! The author is allowed to issue one block but not more during that slot,
//! and it will be built upon the longest valid chain that has been seen.
//!
//! Blocks from future steps will be either deferred or rejected depending on how
//! far in the future they are.
//!
//! NOTE: Aura itself is designed to be generic over the crypto used.
#![forbid(unsafe_code)]
// #![allow(unused_imports)]

// mod aux_schema;
mod import_queue;
mod slot_worker;
mod slots;
// mod finalizer;

use std::{
	convert::{TryFrom, TryInto},
	fmt::Debug,
	hash::Hash,
	marker::PhantomData,
	pin::Pin,
	sync::Arc,
	// time::Duration,
	// thread,
	// str::FromStr,
};

use futures::{
	// future::Either,
	// channel::{oneshot, mpsc}, 
	// select, 
	// future,
	stream::StreamExt,
	prelude::*,
};
// use futures_timer::Delay;
// use rand::Rng;

use log::{debug, trace};

use codec::{Codec, Decode, Encode};

use slot_worker::{
	BackoffAuthoringBlocksStrategy, InherentDataProviderExt, SlotInfo, StorageChanges,
	ElectionWeightInfo,
};

use sc_client_api::{
	backend::{AuxStore, Backend as ClientBackend, Finalizer},
	BlockOf, UsageProvider, BlockchainEvents, ImportNotifications
};
use sc_consensus::{BlockImport, BlockImportParams, ForkChoiceStrategy, StateAction};
use sc_telemetry::{TelemetryHandle};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppKey, AppPublic};
use sp_blockchain::{HeaderBackend, Result as CResult};
use sp_consensus::{
	BlockOrigin, CanAuthorWith, Environment, Error as ConsensusError, Proposer, SelectChain,
	VoteData, ElectionData, VoteElectionRequest,
};
use sp_consensus_slots::Slot;
use sp_core::{
	crypto::{Pair, Public},
	hexdisplay::HexDisplay,
};
use sp_inherents::CreateInherentDataProviders;

use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr, vrf::VRFSignature};
use schnorrkel::vrf::{VRFOutput, VRFProof};
// use sp_keystore::vrf::{VRFTranscriptData, VRFTranscriptValue};
// pub use merlin::Transcript;
use schnorrkel::{keys::PublicKey};

use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header, Member, NumberFor, Zero},
	DigestItem,
};

pub use import_queue::{
	build_verifier, import_queue, AuraVerifier, BuildVerifierParams, CheckForEquivocation,
	ImportQueueParams,
};
pub use slot_worker::SlotProportion;
pub use sp_consensus::SyncOracle;
pub use sp_consensus_vote_election::{
	digests::{CompatibleDigestItem, PreDigest},
	inherents::{InherentDataProvider, InherentType as AuraInherent, INHERENT_IDENTIFIER},
	AuraApi, ConsensusLog, 
	make_transcript, make_transcript_data, VOTE_VRF_PREFIX,

};
// use num_bigint::BigUint;

type AuthorityId<P> = <P as Pair>::Public;

/// Slot duration type for Aura.
pub type SlotDuration = slot_worker::SlotDuration<sp_consensus_vote_election::SlotDuration>;

pub const MAX_VOTE_RANK :usize = 5;
pub const COMMITTEE_TIMEOUT :u64 = 5;
// pub const VOTE_ENGINE_ID : ConsensusEngineId = *b"VOTE";
// pub const VOTE_VRF_PREFIX: &[u8] = b"substrate-vote-vrf";

/// Get type of `SlotDuration` for Aura.
pub fn slot_duration<A, B, C>(client: &C) -> CResult<SlotDuration>
where
	A: Codec,
	B: BlockT,
	C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
	C::Api: AuraApi<B, A>,
{
	// SlotDuration::get_or_compute(client, |a, b| a.slot_duration(b).map_err(Into::into))
	let best_block_id = BlockId::Hash(client.usage_info().chain.best_hash);
	let slot_duration = client.runtime_api().slot_duration(&best_block_id)?;

	Ok(SlotDuration::new(slot_duration))
}

/// Get slot author for given block along with authorities.
// fn slot_author<P: Pair>(slot: Slot, authorities: &[AuthorityId<P>]) -> Option<&AuthorityId<P>> {
// 	if authorities.is_empty() {
// 		return None
// 	}

// 	let idx = *slot % (authorities.len() as u64);
// 	assert!(
// 		idx <= usize::MAX as u64,
// 		"It is impossible to have a vector with length beyond the address space; qed",
// 	);

// 	let current_author = authorities.get(idx as usize).expect(
// 		"authorities not empty; index constrained to list length;this is a valid index; qed",
// 	);

// 	Some(current_author)
// }

/// Parameters of [`start_aura`].
pub struct StartAuraParams<C, SC, I, PF, SO, L, CIDP, BS, CAW> {
	/// The duration of a slot.
	pub slot_duration: SlotDuration,
	/// The client to interact with the chain.
	pub client: Arc<C>,
	/// A select chain implementation to select the best block.
	pub select_chain: SC,
	/// The block import.
	pub block_import: I,
	/// The proposer factory to build proposer instances.
	pub proposer_factory: PF,
	/// The sync oracle that can give us the current sync status.
	pub sync_oracle: SO,
	/// Hook into the sync module to control the justification sync process.
	pub justification_sync_link: L,
	/// Something that can create the inherent data providers.
	pub create_inherent_data_providers: CIDP,
	/// Should we force the authoring of blocks?
	pub force_authoring: bool,
	/// The backoff strategy when we miss slots.
	pub backoff_authoring_blocks: Option<BS>,
	/// The keystore used by the node.
	pub keystore: SyncCryptoStorePtr,
	/// Can we author a block with this node?
	pub can_author_with: CAW,
	/// The proportion of the slot dedicated to proposing.
	///
	/// The block proposing will be limited to this proportion of the slot from the starting of the
	/// slot. However, the proposing can still take longer when there is some lenience factor applied,
	/// because there were no blocks produced for some slots.
	pub block_proposal_slot_portion: SlotProportion,
	/// The maximum proportion of the slot dedicated to proposing with any lenience factor applied
	/// due to no blocks being produced.
	pub max_block_proposal_slot_portion: Option<SlotProportion>,
	/// Telemetry instance used to report telemetry metrics.
	pub telemetry: Option<TelemetryHandle>,
}

/// Start the aura worker. The returned future should be run in a futures executor.
pub fn start_ve_author<P, B, C, SC, I, PF, SO, L, CIDP, BS, CAW, Error>(
	StartAuraParams {
		slot_duration,
		client,
		select_chain,
		block_import,
		proposer_factory,
		sync_oracle,
		justification_sync_link,
		create_inherent_data_providers,
		force_authoring,
		backoff_authoring_blocks,
		keystore,
		can_author_with,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
		telemetry,
	}: StartAuraParams<C, SC, I, PF, SO, L, CIDP, BS, CAW>,
) -> Result<impl Future<Output = ()>, sp_consensus::Error>
where
	P: Pair + Send + Sync,
	P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
	B: BlockT,
	C: ProvideRuntimeApi<B> 
		+ BlockchainEvents<B> 
		+ BlockOf 
		// + ProvideCache<B> 
		+ AuxStore 
		+ HeaderBackend<B> 
		+ Send 
		+ Sync
		+ 'static,
	// C: ProvideRuntimeApi<B> + BlockOf + ProvideCache<B> + AuxStore + HeaderBackend<B> + Send + Sync,
	C::Api: AuraApi<B, AuthorityId<P>>,
	SC: SelectChain<B>,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	L: sc_consensus::JustificationSyncLink<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	CAW: CanAuthorWith<B> + Send,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
	// Ok(sc_consensus_aura_slots::aura_slot_worker_2(client, select_chain))

	let worker = build_aura_worker::<P, _, _, _, _, _, _, _, _>(BuildAuraWorkerParams {
		client: client.clone(),
		block_import,
		proposer_factory,
		keystore,
		sync_oracle: sync_oracle.clone(),
		justification_sync_link,
		force_authoring,
		backoff_authoring_blocks,
		telemetry,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
	});

	Ok(slot_worker::ve_author_worker(
		slot_duration,
		client.clone(),
		select_chain,
		worker,
		sync_oracle,
		create_inherent_data_providers,
		can_author_with,
	))
}


/// Start the aura worker. The returned future should be run in a futures executor.
pub fn start_ve_committee<P, B, C, SC, I, PF, SO, L, CIDP, BS, CAW, Error>(
	StartAuraParams {
		slot_duration,
		client,
		select_chain,
		block_import,
		proposer_factory,
		sync_oracle,
		justification_sync_link,
		create_inherent_data_providers,
		force_authoring,
		backoff_authoring_blocks,
		keystore,
		can_author_with,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
		telemetry,
	}: StartAuraParams<C, SC, I, PF, SO, L, CIDP, BS, CAW>,
) -> Result<impl Future<Output = ()>, sp_consensus::Error>
where
	P: Pair + Send + Sync,
	P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
	B: BlockT,
	C: ProvideRuntimeApi<B> 
		+ BlockchainEvents<B> 
		+ BlockOf 
		// + ProvideCache<B> 
		+ AuxStore 
		+ HeaderBackend<B> 
		+ Send 
		+ Sync
		+ 'static,
	// C: ProvideRuntimeApi<B> + BlockOf + ProvideCache<B> + AuxStore + HeaderBackend<B> + Send + Sync,
	C::Api: AuraApi<B, AuthorityId<P>>,
	SC: SelectChain<B>,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	L: sc_consensus::JustificationSyncLink<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	CAW: CanAuthorWith<B> + Send,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
	// Ok(sc_consensus_aura_slots::aura_slot_worker_2(client, select_chain))

	let worker = build_aura_worker::<P, _, _, _, _, _, _, _, _>(BuildAuraWorkerParams {
		client: client.clone(),
		block_import,
		proposer_factory,
		keystore,
		sync_oracle: sync_oracle.clone(),
		justification_sync_link,
		force_authoring,
		backoff_authoring_blocks,
		telemetry,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
	});

	Ok(slot_worker::ve_committee_worker(
		slot_duration,
		client.clone(),
		select_chain,
		worker,
		sync_oracle,
		create_inherent_data_providers,
		can_author_with,
	))
}


/// Start the aura worker. The returned future should be run in a futures executor.
pub fn start_aura<P, B, C, SC, I, PF, SO, L, CIDP, BS, CAW, Error>(
	StartAuraParams {
		slot_duration,
		client,
		select_chain,
		block_import,
		proposer_factory,
		sync_oracle,
		justification_sync_link,
		create_inherent_data_providers,
		force_authoring,
		backoff_authoring_blocks,
		keystore,
		can_author_with,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
		telemetry,
	}: StartAuraParams<C, SC, I, PF, SO, L, CIDP, BS, CAW>,
) -> Result<impl Future<Output = ()>, sp_consensus::Error>
where
	P: Pair + Send + Sync,
	P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
	B: BlockT,
	C: ProvideRuntimeApi<B> 
		+ BlockchainEvents<B> 
		+ BlockOf 
		// + ProvideCache<B> 
		+ AuxStore 
		+ HeaderBackend<B> 
		+ Send 
		+ Sync
		+ 'static,
	// C: ProvideRuntimeApi<B> + BlockOf + ProvideCache<B> + AuxStore + HeaderBackend<B> + Send + Sync,
	C::Api: AuraApi<B, AuthorityId<P>>,
	SC: SelectChain<B>,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	SO: SyncOracle<B> + Send + Sync + Clone,
	L: sc_consensus::JustificationSyncLink<B>,
	CIDP: CreateInherentDataProviders<B, ()> + Send,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	CAW: CanAuthorWith<B> + Send,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
	// Ok(sc_consensus_aura_slots::aura_slot_worker_2(client, select_chain))

	let worker = build_aura_worker::<P, _, _, _, _, _, _, _, _>(BuildAuraWorkerParams {
		client: client.clone(),
		block_import,
		proposer_factory,
		keystore,
		sync_oracle: sync_oracle.clone(),
		justification_sync_link,
		force_authoring,
		backoff_authoring_blocks,
		telemetry,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
	});

	Ok(slot_worker::start_slot_worker(
		slot_duration,
		client.clone(),
		select_chain,
		worker,
		sync_oracle,
		create_inherent_data_providers,
		can_author_with,
	))
}

pub async fn run_simple_finalizer<A, B, C, CB, P>(client: Arc<C>)
where
    A: Codec + Debug,
    B: BlockT,
	CB: ClientBackend<B>,
    C: BlockchainEvents<B> + Finalizer<B, CB> + ProvideRuntimeApi<B> + BlockOf + Sync,
	C::Api: AuraApi<B, A>,
	P: Pair + Send + Sync,
	P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
{
	let mut imported_blocks_stream = client.import_notification_stream();
	let mut pre_finalize_vec = vec![];

    loop{
        if let Some(block)= imported_blocks_stream.next().await{

            // min_election_weight: authority_len, MAX_VOTE_RANK
            if let Ok(committee_vec) = authorities(client.as_ref(), &BlockId::Hash(block.hash)){
                let min_election_weight = caculate_min_election_weight(committee_vec.len(), MAX_VOTE_RANK);

				if let Ok(weight) = caculate_block_weight::<A, B, P::Signature, C>(&block.header, client.as_ref()){

					if weight <= min_election_weight{
						pre_finalize_vec.push(block.hash);
						// log::info!(
						// 	"⇩ Finalizer: buffer finalize block({}): #{} ({})",
						// 	pre_finalize_vec.len(),
						// 	block.header.number(),
						// 	block.hash
						// );
						while pre_finalize_vec.len() > 3{
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

/// Parameters of [`build_aura_worker`].
pub struct BuildAuraWorkerParams<C, I, PF, SO, L, BS> {
	/// The client to interact with the chain.
	pub client: Arc<C>,
	/// The block import.
	pub block_import: I,
	/// The proposer factory to build proposer instances.
	pub proposer_factory: PF,
	/// The sync oracle that can give us the current sync status.
	pub sync_oracle: SO,
	/// Hook into the sync module to control the justification sync process.
	pub justification_sync_link: L,
	/// Should we force the authoring of blocks?
	pub force_authoring: bool,
	/// The backoff strategy when we miss slots.
	pub backoff_authoring_blocks: Option<BS>,
	/// The keystore used by the node.
	pub keystore: SyncCryptoStorePtr,
	/// The proportion of the slot dedicated to proposing.
	///
	/// The block proposing will be limited to this proportion of the slot from the starting of the
	/// slot. However, the proposing can still take longer when there is some lenience factor applied,
	/// because there were no blocks produced for some slots.
	pub block_proposal_slot_portion: SlotProportion,
	/// The maximum proportion of the slot dedicated to proposing with any lenience factor applied
	/// due to no blocks being produced.
	pub max_block_proposal_slot_portion: Option<SlotProportion>,
	/// Telemetry instance used to report telemetry metrics.
	pub telemetry: Option<TelemetryHandle>,
}

/// Build the aura worker.
///
/// The caller is responsible for running this worker, otherwise it will do nothing.
pub fn build_aura_worker<P, B, C, PF, I, SO, L, BS, Error>(
	BuildAuraWorkerParams {
		client,
		block_import,
		proposer_factory,
		sync_oracle,
		justification_sync_link,
		backoff_authoring_blocks,
		keystore,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
		telemetry,
		force_authoring,
	}: BuildAuraWorkerParams<C, I, PF, SO, L, BS>,
) -> impl slot_worker::SimpleSlotWorker<B>
// ) -> impl sc_consensus_aura_slots::SlotWorker<B, <PF::Proposer as Proposer<B>>::Proof>
where
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + AuxStore + HeaderBackend<B> + Send + Sync,
	C::Api: AuraApi<B, AuthorityId<P>>,
	PF: Environment<B, Error = Error> + Send + Sync + 'static,
	PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
	SO: SyncOracle<B> + Send + Sync + Clone,
	L: sc_consensus::JustificationSyncLink<B>,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
{
	AuraWorker {
		client,
		block_import,
		env: proposer_factory,
		keystore,
		sync_oracle,
		justification_sync_link,
		force_authoring,
		backoff_authoring_blocks,
		telemetry,
		block_proposal_slot_portion,
		max_block_proposal_slot_portion,
		_key_type: PhantomData::<P>,
	}
}

struct AuraWorker<C, E, I, P, SO, L, BS> {
	client: Arc<C>,
	block_import: I,
	env: E,
	keystore: SyncCryptoStorePtr,
	sync_oracle: SO,
	justification_sync_link: L,
	force_authoring: bool,
	backoff_authoring_blocks: Option<BS>,
	block_proposal_slot_portion: SlotProportion,
	max_block_proposal_slot_portion: Option<SlotProportion>,
	telemetry: Option<TelemetryHandle>,
	_key_type: PhantomData<P>,
}

#[async_trait::async_trait]
impl<B, C, E, I, P, Error, SO, L, BS> slot_worker::SimpleSlotWorker<B>
	for AuraWorker<C, E, I, P, SO, L, BS>
where
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + BlockOf + HeaderBackend<B> + Sync,
	// C: ProvideRuntimeApi<B> + BlockOf + ProvideCache<B> + HeaderBackend<B> + Sync,
	C::Api: AuraApi<B, AuthorityId<P>>,
	E: Environment<B, Error = Error>,
	E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
	I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Public + Member + Encode + Decode + Hash,
	P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
	SO: SyncOracle<B> + Send + Clone,
	L: sc_consensus::JustificationSyncLink<B>,
	BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
	Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
	type BlockImport = I;
	type SyncOracle = SO;
	type JustificationSyncLink = L;
	type CreateProposer =
		Pin<Box<dyn Future<Output = Result<E::Proposer, sp_consensus::Error>> + Send + 'static>>;
	type Proposer = E::Proposer;
	type Claim = (PreDigest, P::Public);
	// type Claim = P::Public;
	type EpochData = Vec<AuthorityId<P>>;
	// type BlockchainEvents = BlockchainEvents<B>;

	fn logging_target(&self) -> &'static str {
		"aura"
	}

	fn block_import(&mut self) -> &mut Self::BlockImport {
		&mut self.block_import
	}

	// fn block_chain_events(&self)->BlockchainEvents{
	// 	self.client
	// }

	fn block_notification_stream(&self)->ImportNotifications<B>{
		self.client.import_notification_stream()
		// &self.client.import_notification_stream()
	}

	fn epoch_data(
		&self,
		header: &B::Header,
		_slot: Slot,
	) -> Result<Self::EpochData, sp_consensus::Error> {
		authorities(self.client.as_ref(), &BlockId::Hash(header.hash()))
	}

	fn authorities_len(&self, epoch_data: &Self::EpochData) -> Option<usize> {
		Some(epoch_data.len())
	}

	// fn claim_slot(
	// 	&mut self,
	// 	_header: &B::Header,
	// 	slot: Slot,
	// 	_epoch_data: &Self::EpochData,
	// ) -> Option<Self::Claim> {
	// 	let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
	// 		&*self.keystore, 
	// 		sp_application_crypto::key_types::AURA
	// 	);

	// 	if sr25519_public_keys.len() > 0 {

	// 		let pub_bytes = sr25519_public_keys[0].to_raw_vec();
	// 		if let Ok(author) = <AuthorityId<P> as Decode>::decode(&mut pub_bytes.as_slice()){
	// 			// let pre_digest = PreDigest{slot: slot, public: author.to_raw_vec()};
	// 			let pre_digest = PreDigest{
	// 				slot,
	// 				rand_bytes: vec![],
	// 				pub_bytes: author.to_raw_vec(),
	// 				election_bytes: vec![],
	// 			};
	// 			return Some((pre_digest, author.clone()));
	// 		}
	// 	}

	// 	None
	// }

	fn claim_slot_v2(
		&mut self,
		slot: Slot,
		vrf_sig: &VRFSignature,
		election_vec: Vec<ElectionData<B>>,
	) -> Option<Self::Claim> {
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA
		);

		if sr25519_public_keys.len() == 1{

			let pub_bytes = sr25519_public_keys[0].to_raw_vec();
			if let Ok(author) = <AuthorityId<P> as Decode>::decode(&mut pub_bytes.as_slice()){
				let pre_digest = PreDigest{
					slot,
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

	fn is_committee(&mut self, hash: &B::Hash)->bool{
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

	// add by user
	fn notify_slot(&self, _header: &B::Header, _slot: Slot, _epoch_data: &Self::EpochData) {
	}

	fn pre_digest_data(
		&self,
		_slot: Slot,
		claim: &Self::Claim,
	) -> Vec<sp_runtime::DigestItem<B::Hash>> {
		// vec![<DigestItemFor<B> as CompatibleDigestItem<P::Signature>>::aura_pre_digest(slot.clone())]
		vec![<DigestItem<B::Hash> as CompatibleDigestItem<P::Signature>>::aura_pre_digest(claim.0.clone())]
	}

	fn block_import_params(
		&self,
	) -> Box<
		dyn Fn(
				B::Header,
				&B::Hash,
				Vec<B::Extrinsic>,
				StorageChanges<sp_api::TransactionFor<C, B>, B>,
				Self::Claim,
				Self::EpochData,
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
				// <DigestItem as CompatibleDigestItem<P::Signature>>::aura_seal(signature);

			let mut import_block = BlockImportParams::new(BlockOrigin::Own, header);
			import_block.post_digests.push(signature_digest_item);
			import_block.body = Some(body);
			import_block.state_action =
				StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
			import_block.fork_choice = Some(ForkChoiceStrategy::LongestChain);

			Ok(import_block)
		})
	}

	fn force_authoring(&self) -> bool {
		self.force_authoring
	}

	fn should_backoff(&self, slot: Slot, chain_head: &B::Header) -> bool {
		if let Some(ref strategy) = self.backoff_authoring_blocks {
			if let Ok(pre_digest) = find_pre_digest::<B, P::Signature>(chain_head) {
				let chain_head_slot = pre_digest.slot;
				return strategy.should_backoff(
					*chain_head.number(),
					chain_head_slot,
					self.client.info().finalized_number,
					slot,
					self.logging_target(),
				)
			}
		}
		false
	}

	fn sync_oracle(&mut self) -> &mut Self::SyncOracle {
		&mut self.sync_oracle
	}

	fn justification_sync_link(&mut self) -> &mut Self::JustificationSyncLink {
		&mut self.justification_sync_link
	}

	fn proposer(&mut self, block: &B::Header) -> Self::CreateProposer {
		Box::pin(
			self.env
				.init(block)
				.map_err(|e| sp_consensus::Error::ClientImport(format!("{:?}", e)).into()),
		)
	}

	fn telemetry(&self) -> Option<TelemetryHandle> {
		self.telemetry.clone()
	}

	fn proposing_remaining_duration(&self, slot_info: &SlotInfo<B>) -> std::time::Duration {
		let parent_slot = find_pre_digest::<B, P::Signature>(&slot_info.chain_head).ok().map(|d| d.slot);

		slot_worker::proposing_remaining_duration(
			parent_slot,
			slot_info,
			&self.block_proposal_slot_portion,
			self.max_block_proposal_slot_portion.as_ref(),
			slot_worker::SlotLenienceType::Exponential,
			self.logging_target(),
		)
	}

	fn generate_author_vrf_data(&mut self, cur_hash: &B::Hash)->Result<(u128, VRFSignature), String>{
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

	// fn propagate_vote(&mut self){
	fn propagate_vote(&mut self, vrf_sig :&VRFSignature, cur_hash: &B::Hash){
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA
		);

		if sr25519_public_keys.len() == 0{
			log::info!("No public key");
			return;
		}

		let vote_data = VoteData::<B>{
			block_hash: cur_hash.clone(),
			vrf_output_bytes: vrf_sig.output.to_bytes().encode(),
			vrf_proof_bytes: vrf_sig.proof.to_bytes().encode(),
			pub_bytes: sr25519_public_keys[0].to_raw_vec(),
		};
		self.sync_oracle.ve_request(VoteElectionRequest::PropagateVote(vote_data));
	}

	fn verify_vote(self: &mut Self, vote_data: &VoteData<B>)->Result<u128, String>{
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

	fn verify_election(&mut self, election_data: &ElectionData<B>, &cur_hash: &B::Hash)->bool{
		// hash_verify
		// let cur_hash = header.hash();
		if cur_hash != election_data.block_hash{
			// log::info!("verify_election() failed, hash not eq, cur: {}, recv: {}", cur_hash, election_data.hash);
			return false;
		}

		// if pub_bytes not in committee
		if let Ok(committee_vec) = authorities(self.client.as_ref(), &BlockId::Hash(cur_hash)){
			let mut is_committee = false;
			for committee in committee_vec.iter(){
				if election_data.committee_pub_bytes == committee.to_raw_vec(){
					is_committee |= true; 
					break;
				}
			}
			if is_committee == false{
				log::info!("verify_election() failed, not committee member");
				return false;
			}
		}
		else{
			log::info!("verify_election() failed, get committee member failed");
			return false;
		}

		// check signature
		let ElectionData{block_hash, sig_bytes, vote_list, committee_pub_bytes} = election_data;
		if let Ok(sig) = <P::Signature as Decode>::decode(&mut sig_bytes.as_slice()){
			let mut msg_bytes :Vec<u8> = vec![];
			msg_bytes.extend(block_hash.encode().iter());
			msg_bytes.extend(vote_list.encode().iter());

			let msg = msg_bytes.as_slice();

			if let Ok(verify_public) = <AuthorityId<P> as Decode>::decode(&mut committee_pub_bytes.as_slice()){
				let sign_ret = P::verify(&sig, &msg, &verify_public);
				return sign_ret;
			}
		}
		false
	}

	fn propagate_election(&mut self, block_hash: B::Hash, election_ret: Vec<VoteData<B>>){
		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA
		);

		if sr25519_public_keys.len() == 0{
			log::info!("propagate_election failed");
		}

		let public_type_pair = sr25519_public_keys[0].to_public_crypto_pair();

		let mut pre_election:Vec<u8>= vec![];
		pre_election.extend(block_hash.encode().iter());
		pre_election.extend(election_ret.encode().iter());

		let msg = pre_election.as_slice();

		if let Ok(Some(sig_bytes)) = SyncCryptoStore::sign_with(
			&*self.keystore,
			<AuthorityId<P> as AppKey>::ID,
			&public_type_pair,
			&msg,
		){
			let pub_bytes = sr25519_public_keys[0].to_raw_vec();
			// let election_data = <ElectionData<B>>::new(hash, sig_bytes, election_ret, pub_bytes);
			let election_data = ElectionData::<B>{
				block_hash,
				sig_bytes,
				vote_list: election_ret,
				committee_pub_bytes: pub_bytes
			};
			self.sync_oracle.ve_request(VoteElectionRequest::PropagateElection(election_data));
		}
	}

	fn caculate_min_max_weight(&mut self, header: &B::Header)->Result<(u64, u64), String>{
		match authorities(self.client.as_ref(), &BlockId::Hash(header.hash())){
			Ok(author_vec) => {
				let author_count = author_vec.len();
				let max_vote_rank = MAX_VOTE_RANK;
				Ok(caculate_min_max_election_weight(author_count, max_vote_rank))

				// let min_value = caculate_min_election_value(author_count, max_vote_rank);
				// Ok(min_value)
			},
			Err(e)=>{
				Err(format!("get pallet author failed: {}", e))
			},
		}
	}

	fn caculate_weight_info_from_header(
		&mut self,
		header:&B::Header
	)->Result<ElectionWeightInfo, String>{
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

		let election_weight = caculate_weight_from_elections(&pub_bytes, &election_vec, committee_count);

		Ok(ElectionWeightInfo{
			weight: election_weight,
			random: vrf_num,
		})
	}

	fn caculate_elections_weight(
		&mut self,
		header: &B::Header,
		election_vec: &Vec<ElectionData<B>>,
	)->Result<u64, String> {
		let committee_list = authorities(self.client.as_ref(), &BlockId::Hash(header.hash()))
			.map_err(|e|format!("Get committee from pallet failed: {}", e))?;

		let sr25519_public_keys = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore, 
			sp_application_crypto::key_types::AURA,
		);

		if sr25519_public_keys.len() == 0{
			return Err(format!("No public key"));
		}

		let pub_bytes = sr25519_public_keys[0].to_raw_vec();

		// let weight = caculate_weight_from_elections(pub_bytes, election_vec);
		// Ok(weight)

		let mut rank_vec = vec![];
		for election in election_vec.iter(){
			let rank = match election.vote_list.iter().position(|vote|vote.pub_bytes == pub_bytes){
				Some(x) if x < MAX_VOTE_RANK => x,
				_ => MAX_VOTE_RANK,
			};
			rank_vec.push(rank);
		}

		let committee_count = committee_list.len();
		while rank_vec.len() < committee_count{
			rank_vec.push(MAX_VOTE_RANK);
		}

		rank_vec.sort();
		if let Some(election) = election_vec.last(){
			// let pub_keys = election.committee_pub_bytes;
			let account_str = format!("{}", HexDisplay::from(&election.committee_pub_bytes));
			log::info!(
				target: "vote",
				// "{:?}, election result", 
				// rank_vec,
				"{:?}, election result,  #0x{}...{}", 
				rank_vec,
				// HexDisplay::from(&election.committee_pub_bytes)
				&account_str[0..4],
				&account_str[60..],
			);
		}
		else{
			log::info!( target: "vote", "{:?}, election result", rank_vec);
		}
		// log::info!("0x{}", HexDisplay::from(committee_pub_bytes));
		// let weight = caculate_weight_from_ranks(&rank_vec, MAX_VOTE_RANK);

		let weight = caculate_weight_from_elections(&pub_bytes, &election_vec, committee_count);
		Ok(weight)
	}
}

pub fn caculate_block_weight<A, B, S, C>(header: &B::Header, client: &C)->Result<u64, Error<B>>
where
	A: Codec + Debug,
	B: BlockT,
	S: Codec,
	C: ProvideRuntimeApi<B> + BlockOf,
	C::Api: AuraApi<B, A>,
{
	let committee_vec = authorities(client, &BlockId::Hash(header.hash()))
		.map_err(|_|Error::NoCommitteeFound)?;

	let committee_count = committee_vec.len();

	let pre_digest = find_pre_digest::<B, S>(header)
		.map_err(|_|Error::NoDigestFound)?;

	let pub_bytes = pre_digest.pub_key_bytes;

	let election_vec = <Vec<ElectionData<B>> as Decode>::decode(&mut pre_digest.election_bytes.as_slice())
		.map_err(|_|Error::ElectionDataDecodeFailed)?;
	
	let block_election_weight = caculate_weight_from_elections(&pub_bytes, &election_vec, committee_count);

	Ok(block_election_weight)
}

fn caculate_weight_from_elections<B: BlockT>(pub_bytes: &Vec<u8>, election_vec: &Vec<ElectionData<B>>, committee_count: usize)->u64{
	let mut rank_vec = vec![];
	for election in election_vec.iter(){
		let rank = match election.vote_list.iter().position(|vote| vote.pub_bytes == *pub_bytes){
			Some(x) if x < MAX_VOTE_RANK =>x,
			// None => MAX_VOTE_RANK,
			_ => MAX_VOTE_RANK,
		};
		rank_vec.push(rank);
	}

	while rank_vec.len() < committee_count{
		rank_vec.push(MAX_VOTE_RANK);
	}

	rank_vec.sort();
	let weight = caculate_weight_from_ranks(&rank_vec, MAX_VOTE_RANK);
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

fn caculate_min_max_election_weight(committee_count: usize, max_vote_rank: usize)->(u64, u64){
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

fn caculate_max_election_weight(committee_count: usize, max_vote_rank: usize)->u64{
	let mut ret = 1;

	let mut i = 0;
	while i < committee_count{
		ret *= (max_vote_rank+1) as u64;
		i += 1;
	}
	ret-1
}

fn aura_err<B: BlockT>(error: Error<B>) -> Error<B> {
	debug!(target: "aura", "{}", error);
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
			slot: 0.into(), 
			pub_key_bytes: vec![],
			vrf_output_bytes: vec![],
			vrf_proof_bytes: vec![],
			election_bytes: vec![],
		})
	}

	let mut pre_digest: Option<_> = None;
	for log in header.digest().logs() {
		trace!(target: "aura", "Checking log {:?}", log);
		match (CompatibleDigestItem::<Signature>::as_aura_pre_digest(log), pre_digest.is_some()) {
		// match (log.as_aura_pre_digest(), pre_digest.is_some()){
			(Some(_), true) => return Err(aura_err(Error::MultipleHeaders)),
			(None, _) => trace!(target: "babe", "Ignoring digest not meant for us"),
			(s, false) => pre_digest = s,
		}
	}
	pre_digest.ok_or_else(|| aura_err(Error::NoDigestFound))
}

pub fn authorities<A, B, C>(client: &C, at: &BlockId<B>) -> Result<Vec<A>, ConsensusError>
where
	A: Codec + Debug,
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockOf,
	C::Api: AuraApi<B, A>,
{
	client
		.runtime_api()
		.authorities(at)
		.ok()
		.ok_or_else(|| sp_consensus::Error::InvalidAuthoritiesSet.into())
}

#[cfg(test)]
mod tests {
	use super::*;
	use parking_lot::Mutex;
	use sc_block_builder::BlockBuilderProvider;
	use sc_client_api::BlockchainEvents;
	use sc_consensus::BoxJustificationImport;
	use sc_consensus_aura_slots::{BackoffAuthoringOnFinalizedHeadLagging, SimpleSlotWorker};
	use sc_keystore::LocalKeystore;
	use sc_network::config::ProtocolConfig;
	use sc_network_test::{Block as TestBlock, *};
	use sp_application_crypto::key_types::AURA;
	use sp_consensus::{
		AlwaysCanAuthor, DisableProofRecording, NoNetwork as DummyOracle, Proposal, SlotData,
	};
	use sp_consensus_aura::sr25519::AuthorityPair;
	use sp_inherents::InherentData;
	use sp_keyring::sr25519::Keyring;
	use sp_runtime::traits::{Block as BlockT, DigestFor, Header as _};
	use sp_timestamp::InherentDataProvider as TimestampInherentDataProvider;
	use std::{
		task::Poll,
		time::{Duration, Instant},
	};
	use substrate_test_runtime_client::{
		runtime::{Header, H256},
		TestClient,
	};

	type Error = sp_blockchain::Error;

	struct DummyFactory(Arc<TestClient>);
	struct DummyProposer(u64, Arc<TestClient>);

	impl Environment<TestBlock> for DummyFactory {
		type Proposer = DummyProposer;
		type CreateProposer = futures::future::Ready<Result<DummyProposer, Error>>;
		type Error = Error;

		fn init(&mut self, parent_header: &<TestBlock as BlockT>::Header) -> Self::CreateProposer {
			futures::future::ready(Ok(DummyProposer(parent_header.number + 1, self.0.clone())))
		}
	}

	impl Proposer<TestBlock> for DummyProposer {
		type Error = Error;
		type Transaction =
			sc_client_api::TransactionFor<substrate_test_runtime_client::Backend, TestBlock>;
		type Proposal = future::Ready<Result<Proposal<TestBlock, Self::Transaction, ()>, Error>>;
		type ProofRecording = DisableProofRecording;
		type Proof = ();

		fn propose(
			self,
			_: InherentData,
			digests: DigestFor<TestBlock>,
			_: Duration,
			_: Option<usize>,
		) -> Self::Proposal {
			let r = self.1.new_block(digests).unwrap().build().map_err(|e| e.into());

			future::ready(r.map(|b| Proposal {
				block: b.block,
				proof: (),
				storage_changes: b.storage_changes,
			}))
		}
	}

	const SLOT_DURATION: u64 = 1000;

	type AuraVerifier = import_queue::AuraVerifier<
		PeersFullClient,
		AuthorityPair,
		AlwaysCanAuthor,
		Box<
			dyn CreateInherentDataProviders<
				TestBlock,
				(),
				InherentDataProviders = (TimestampInherentDataProvider, InherentDataProvider),
			>,
		>,
	>;
	type AuraPeer = Peer<(), PeersClient>;

	pub struct AuraTestNet {
		peers: Vec<AuraPeer>,
	}

	impl TestNetFactory for AuraTestNet {
		type Verifier = AuraVerifier;
		type PeerData = ();
		type BlockImport = PeersClient;

		/// Create new test network with peers and given config.
		fn from_config(_config: &ProtocolConfig) -> Self {
			AuraTestNet { peers: Vec::new() }
		}

		fn make_verifier(
			&self,
			client: PeersClient,
			_cfg: &ProtocolConfig,
			_peer_data: &(),
		) -> Self::Verifier {
			match client {
				PeersClient::Full(client, _) => {
					let slot_duration = slot_duration(&*client).expect("slot duration available");

					assert_eq!(slot_duration.slot_duration().as_millis() as u64, SLOT_DURATION);
					import_queue::AuraVerifier::new(
						client,
						Box::new(|_, _| async {
							let timestamp = TimestampInherentDataProvider::from_system_time();
							let slot = InherentDataProvider::from_timestamp_and_duration(
								*timestamp,
								Duration::from_secs(6),
							);

							Ok((timestamp, slot))
						}),
						AlwaysCanAuthor,
						CheckForEquivocation::Yes,
						None,
					)
				},
				PeersClient::Light(_, _) => unreachable!("No (yet) tests for light client + Aura"),
			}
		}

		fn make_block_import(
			&self,
			client: PeersClient,
		) -> (
			BlockImportAdapter<Self::BlockImport>,
			Option<BoxJustificationImport<Block>>,
			Self::PeerData,
		) {
			(client.as_block_import(), None, ())
		}

		fn peer(&mut self, i: usize) -> &mut AuraPeer {
			&mut self.peers[i]
		}

		fn peers(&self) -> &Vec<AuraPeer> {
			&self.peers
		}
		fn mut_peers<F: FnOnce(&mut Vec<AuraPeer>)>(&mut self, closure: F) {
			closure(&mut self.peers);
		}
	}

	#[test]
	fn authoring_blocks() {
		sp_tracing::try_init_simple();
		let net = AuraTestNet::new(3);

		let peers = &[(0, Keyring::Alice), (1, Keyring::Bob), (2, Keyring::Charlie)];

		let net = Arc::new(Mutex::new(net));
		let mut import_notifications = Vec::new();
		let mut aura_futures = Vec::new();

		let mut keystore_paths = Vec::new();
		for (peer_id, key) in peers {
			let mut net = net.lock();
			let peer = net.peer(*peer_id);
			let client = peer.client().as_full().expect("full clients are created").clone();
			let select_chain = peer.select_chain().expect("full client has a select chain");
			let keystore_path = tempfile::tempdir().expect("Creates keystore path");
			let keystore = Arc::new(
				LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore."),
			);

			SyncCryptoStore::sr25519_generate_new(&*keystore, AURA, Some(&key.to_seed()))
				.expect("Creates authority key");
			keystore_paths.push(keystore_path);

			let environ = DummyFactory(client.clone());
			import_notifications.push(
				client
					.import_notification_stream()
					.take_while(|n| {
						future::ready(!(n.origin != BlockOrigin::Own && n.header.number() < &5))
					})
					.for_each(move |_| future::ready(())),
			);

			let slot_duration = slot_duration(&*client).expect("slot duration available");

			aura_futures.push(
				start_aura::<AuthorityPair, _, _, _, _, _, _, _, _, _, _, _>(StartAuraParams {
					slot_duration,
					block_import: client.clone(),
					select_chain,
					client,
					proposer_factory: environ,
					sync_oracle: DummyOracle,
					justification_sync_link: (),
					create_inherent_data_providers: |_, _| async {
						let timestamp = TimestampInherentDataProvider::from_system_time();
						let slot = InherentDataProvider::from_timestamp_and_duration(
							*timestamp,
							Duration::from_secs(6),
						);

						Ok((timestamp, slot))
					},
					force_authoring: false,
					backoff_authoring_blocks: Some(
						BackoffAuthoringOnFinalizedHeadLagging::default(),
					),
					keystore,
					can_author_with: sp_consensus::AlwaysCanAuthor,
					block_proposal_slot_portion: SlotProportion::new(0.5),
					max_block_proposal_slot_portion: None,
					telemetry: None,
				})
				.expect("Starts aura"),
			);
		}

		futures::executor::block_on(future::select(
			future::poll_fn(move |cx| {
				net.lock().poll(cx);
				Poll::<()>::Pending
			}),
			future::select(future::join_all(aura_futures), future::join_all(import_notifications)),
		));
	}

	#[test]
	fn authorities_call_works() {
		let client = substrate_test_runtime_client::new();

		assert_eq!(client.chain_info().best_number, 0);
		assert_eq!(
			authorities(&client, &BlockId::Number(0)).unwrap(),
			vec![
				Keyring::Alice.public().into(),
				Keyring::Bob.public().into(),
				Keyring::Charlie.public().into()
			]
		);
	}

	#[test]
	fn current_node_authority_should_claim_slot() {
		let net = AuraTestNet::new(4);

		let mut authorities = vec![
			Keyring::Alice.public().into(),
			Keyring::Bob.public().into(),
			Keyring::Charlie.public().into(),
		];

		let keystore_path = tempfile::tempdir().expect("Creates keystore path");
		let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
		let public = SyncCryptoStore::sr25519_generate_new(&keystore, AuthorityPair::ID, None)
			.expect("Key should be created");
		authorities.push(public.into());

		let net = Arc::new(Mutex::new(net));

		let mut net = net.lock();
		let peer = net.peer(3);
		let client = peer.client().as_full().expect("full clients are created").clone();
		let environ = DummyFactory(client.clone());

		let worker = AuraWorker {
			client: client.clone(),
			block_import: client,
			env: environ,
			keystore: keystore.into(),
			sync_oracle: DummyOracle.clone(),
			justification_sync_link: (),
			force_authoring: false,
			backoff_authoring_blocks: Some(BackoffAuthoringOnFinalizedHeadLagging::default()),
			telemetry: None,
			_key_type: PhantomData::<AuthorityPair>,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
		};

		let head = Header::new(
			1,
			H256::from_low_u64_be(0),
			H256::from_low_u64_be(0),
			Default::default(),
			Default::default(),
		);
		assert!(worker.claim_slot(&head, 0.into(), &authorities).is_none());
		assert!(worker.claim_slot(&head, 1.into(), &authorities).is_none());
		assert!(worker.claim_slot(&head, 2.into(), &authorities).is_none());
		assert!(worker.claim_slot(&head, 3.into(), &authorities).is_some());
		assert!(worker.claim_slot(&head, 4.into(), &authorities).is_none());
		assert!(worker.claim_slot(&head, 5.into(), &authorities).is_none());
		assert!(worker.claim_slot(&head, 6.into(), &authorities).is_none());
		assert!(worker.claim_slot(&head, 7.into(), &authorities).is_some());
	}

	#[test]
	fn on_slot_returns_correct_block() {
		let net = AuraTestNet::new(4);

		let keystore_path = tempfile::tempdir().expect("Creates keystore path");
		let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
		SyncCryptoStore::sr25519_generate_new(
			&keystore,
			AuthorityPair::ID,
			Some(&Keyring::Alice.to_seed()),
		)
		.expect("Key should be created");

		let net = Arc::new(Mutex::new(net));

		let mut net = net.lock();
		let peer = net.peer(3);
		let client = peer.client().as_full().expect("full clients are created").clone();
		let environ = DummyFactory(client.clone());

		let mut worker = AuraWorker {
			client: client.clone(),
			block_import: client.clone(),
			env: environ,
			keystore: keystore.into(),
			sync_oracle: DummyOracle.clone(),
			justification_sync_link: (),
			force_authoring: false,
			backoff_authoring_blocks: Option::<()>::None,
			telemetry: None,
			_key_type: PhantomData::<AuthorityPair>,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
		};

		let head = client.header(&BlockId::Number(0)).unwrap().unwrap();

		let res = futures::executor::block_on(worker.on_slot(SlotInfo {
			slot: 0.into(),
			timestamp: 0.into(),
			ends_at: Instant::now() + Duration::from_secs(100),
			inherent_data: InherentData::new(),
			duration: Duration::from_millis(1000),
			chain_head: head,
			block_size_limit: None,
		}))
		.unwrap();

		// The returned block should be imported and we should be able to get its header by now.
		assert!(client.header(&BlockId::Hash(res.block.hash())).unwrap().is_some());
	}
}
