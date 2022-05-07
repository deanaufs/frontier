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

//! Module implementing the logic for verifying and importing AuRa blocks.

use crate::{vote_err, authorities, find_pre_digest, AuthorityId, Error};
use codec::{Codec, Decode, Encode};
use log::{debug, trace};
use prometheus_endpoint::Registry;
use sc_client_api::{backend::AuxStore, BlockOf, UsageProvider};
use sc_consensus::{
	block_import::{BlockImport, BlockImportParams, ForkChoiceStrategy},
	import_queue::{BasicQueue, BoxJustificationImport, DefaultImportQueue, Verifier},
};
// use crate::slot_worker::{CheckedHeader, InherentDataProviderExt};
use crate::worker::{CheckedHeader, InherentDataProviderExt};
use sc_telemetry::{telemetry, TelemetryHandle, CONSENSUS_TRACE};
use sp_api::{ApiExt, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder as BlockBuilderApi;
use sp_blockchain::{
	well_known_cache_keys::{self, Id as CacheKeyId},
	HeaderBackend,
};
use sp_consensus::{CanAuthorWith, Error as ConsensusError, ElectionData};
use sp_consensus_vote_election::{
	digests::CompatibleDigestItem, VoteElectionApi, ConsensusLog,
	// digests::CompatibleDigestItem, inherents::AuraInherentData, AuraApi, ConsensusLog,
	VOTE_ENGINE_ID, make_transcript,
};
// use sp_consensus_slots::Slot;
use sp_core::{crypto::Pair, ExecutionContext};
use sp_inherents::{CreateInherentDataProviders, InherentDataProvider};
use sp_runtime::{
	generic::{BlockId, OpaqueDigestItemId},
	traits::{Block as BlockT, Header},
	DigestItem,
};
use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};
use schnorrkel::vrf::{VRFOutput, VRFProof};

/// check a header has been signed by the right key. If the slot is too far in the future, an error
/// will be returned. If it's successful, returns the pre-header and the digest item
/// containing the seal.
///
/// This digest item will always return `Some` when used with `as_aura_seal`.
fn check_header<C, B: BlockT, P: Pair>(
	mut header: B::Header,
	block_hash: B::Hash,
	parent_hash: B::Hash,
	authorities: &[AuthorityId<P>],
) -> Result<CheckedHeader<B::Header, (DigestItem<B::Hash>, )>, Error<B>>
where
	P::Signature: Codec,
	C: sc_client_api::backend::AuxStore,
	P::Public: Encode + Decode + PartialEq + Clone,
{
	let seal = header.digest_mut().pop().ok_or_else(|| Error::HeaderUnsealed(block_hash))?;

	let sig = seal.as_ve_seal().ok_or_else(|| vote_err(Error::HeaderBadSeal(block_hash)))?;

	let pre_digest = find_pre_digest::<B, P::Signature>(&header)?;

	// verify vrf output
	let transcript = make_transcript(&header.parent_hash().encode());

	let vrf_output = VRFOutput::from_bytes(pre_digest.vrf_output_bytes.as_slice())
		.map_err(|_|Error::VRFOutputDecodeFailed)?;
	
	let vrf_proof = VRFProof::from_bytes(pre_digest.vrf_proof_bytes.as_slice())
		.map_err(|_|Error::VRFProofDecodeFailed)?;

	// verify block vrf
	match schnorrkel::PublicKey::from_bytes(&pre_digest.pub_key_bytes)
		.and_then(|p|{ p.vrf_verify(transcript, &vrf_output, &vrf_proof)}){
			Ok(_)=>{
				// log::info!("check vrf success");
			},
			Err(_)=> return Err(Error::VRFVerifyFailed),
		}

	// verify pre_digest
	let election_bytes = pre_digest.election_bytes;
	let election_vec = match <Vec<ElectionData<B>>>::decode(&mut election_bytes.as_slice()){
		Ok(v)=> v,
		Err(_)=> return Err(Error::ElectionDataDecodeFailed),
	};

	if election_vec.len() == 0{
		return Err(Error::NoElectionDataFound);
	}

	for election in election_vec.iter(){
		let ElectionData{block_hash, sig_bytes, vote_list, committee_pub_bytes} = election;
		let hash = block_hash;
		if *hash != parent_hash{
			log::info!("Bad election, wrong hash, cur: {}, parent_hash: {}", block_hash, parent_hash);
			return Err(Error::BadElection(*block_hash));
		}

		let sig = <P::Signature as Decode>::decode(&mut sig_bytes.as_slice())
			.map_err(|_|Error::BadElectionSignatureBytes)?;

		let mut msg_bytes :Vec<u8> = vec![];
		msg_bytes.extend(hash.encode().iter());
		msg_bytes.extend(vote_list.encode().iter());

		let msg = msg_bytes.as_slice();

		let verify_public = <AuthorityId<P> as Decode>::decode(&mut committee_pub_bytes.as_slice())
			.map_err(|_|Error::BadElectionCommitteeBytes)?;

		if !authorities.contains(&verify_public){
			log::info!("Election not from committee");
			Err(Error::BadElection(*block_hash))?;
		}
		if !P::verify(&sig, &msg, &verify_public){
			log::info!("predigest verify failed");
			Err(Error::BadSignature(*block_hash))?;
		}
	}

	let expected_author = match <AuthorityId<P> as Decode>::decode(&mut pre_digest.pub_key_bytes.as_slice()){
		Ok(author)=> author.clone(),
		Err(_) => return Err(Error::NoDigestFound),
	};

	let pre_hash = header.hash();
	if P::verify(&sig, pre_hash.as_ref(), &expected_author) {
		Ok(CheckedHeader::Checked(header, (seal, )))
	}
	else{
		Err(Error::BadSignature(block_hash))
	}
}

/// A verifier for Aura blocks.
pub struct AuraVerifier<C, P, CAW, CIDP> {
	client: Arc<C>,
	phantom: PhantomData<P>,
	create_inherent_data_providers: CIDP,
	can_author_with: CAW,
	telemetry: Option<TelemetryHandle>,
}

impl<C, P, CAW, CIDP> AuraVerifier<C, P, CAW, CIDP> {
	pub(crate) fn new(
		client: Arc<C>,
		create_inherent_data_providers: CIDP,
		can_author_with: CAW,
		telemetry: Option<TelemetryHandle>,
	) -> Self {
		Self {
			client,
			create_inherent_data_providers,
			can_author_with,
			telemetry,
			phantom: PhantomData,
		}
	}
}

impl<C, P, CAW, CIDP> AuraVerifier<C, P, CAW, CIDP>
where
	P: Send + Sync + 'static,
	CAW: Send + Sync + 'static,
	CIDP: Send,
{
	async fn check_inherents<B: BlockT>(
		&self,
		block: B,
		block_id: BlockId<B>,
		inherent_data: sp_inherents::InherentData,
		create_inherent_data_providers: CIDP::InherentDataProviders,
		execution_context: ExecutionContext,
	) -> Result<(), Error<B>>
	where
		C: ProvideRuntimeApi<B>,
		C::Api: BlockBuilderApi<B>,
		CAW: CanAuthorWith<B>,
		CIDP: CreateInherentDataProviders<B, ()>,
	{
		if let Err(e) = self.can_author_with.can_author_with(&block_id) {
			debug!(
				target: "aura",
				"Skipping `check_inherents` as authoring version is not compatible: {}",
				e,
			);

			return Ok(())
		}

		let inherent_res = self
			.client
			.runtime_api()
			.check_inherents_with_context(&block_id, execution_context, block, inherent_data)
			.map_err(|e| Error::Client(e.into()))?;

		if !inherent_res.ok() {
			for (i, e) in inherent_res.into_errors() {
				match create_inherent_data_providers.try_handle_error(&i, &e).await {
					Some(res) => res.map_err(Error::Inherent)?,
					None => return Err(Error::UnknownInherentError(i)),
				}
			}
		}

		Ok(())
	}
}

#[async_trait::async_trait]
impl<B: BlockT, C, P, CAW, CIDP> Verifier<B> for AuraVerifier<C, P, CAW, CIDP>
where
	C: ProvideRuntimeApi<B> + Send + Sync + sc_client_api::backend::AuxStore + BlockOf,
	C::Api: BlockBuilderApi<B> + VoteElectionApi<B, AuthorityId<P>> + ApiExt<B>,
	P: Pair + Send + Sync + 'static,
	P::Public: Send + Sync + Hash + Eq + Clone + Decode + Encode + Debug + 'static,
	P::Signature: Encode + Decode,
	CAW: CanAuthorWith<B> + Send + Sync + 'static,
	CIDP: CreateInherentDataProviders<B, ()> + Send + Sync,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send + Sync,
{
	async fn verify(
		&mut self,
		mut block: BlockImportParams<B, ()>,
	) -> Result<(BlockImportParams<B, ()>, Option<Vec<(CacheKeyId, Vec<u8>)>>), String> {
		let hash = block.header.hash();
		let parent_hash = *block.header.parent_hash();
		let authorities = authorities(self.client.as_ref(), &BlockId::Hash(parent_hash))
			.map_err(|e| format!("Could not fetch authorities at {:?}: {:?}", parent_hash, e))?;

		let create_inherent_data_providers = self
			.create_inherent_data_providers
			.create_inherent_data_providers(parent_hash, ())
			.await
			.map_err(|e| Error::<B>::Client(sp_blockchain::Error::Application(e)))?;

		let inherent_data = create_inherent_data_providers
			.create_inherent_data()
			.map_err(Error::<B>::Inherent)?;

		// // we add one to allow for some small drift.
		// // FIXME #1019 in the future, alter this queue to allow deferring of
		// // headers
		// let checked_header = check_header::<C, B, P>(
		// 	&self.client,
		// 	slot_now + 1,
		// 	block.header,
		// 	hash,
		// 	&authorities[..],
		// 	self.check_for_equivocation,
		// )

		let checked_header = check_header::<C, B, P>(
			block.header,
			hash,
			parent_hash,
			&authorities,
		)
		.map_err(|e| e.to_string())?;
		match checked_header {
			CheckedHeader::Checked(pre_header, (seal, )) => {
				// if the body is passed through, we need to use the runtime
				// to check that the internally-set timestamp in the inherents
				// actually matches the slot set in the seal.
				if let Some(inner_body) = block.body.take() {
					let new_block = B::new(pre_header.clone(), inner_body);

					// inherent_data.aura_replace_inherent_data(slot);

					// skip the inherents verification if the runtime API is old.
					if self
						.client
						.runtime_api()
						.has_api_with::<dyn BlockBuilderApi<B>, _>(
							&BlockId::Hash(parent_hash),
							|v| v >= 2,
						)
						.map_err(|e| format!("{:?}", e))?
					{
						self.check_inherents(
							new_block.clone(),
							BlockId::Hash(parent_hash),
							inherent_data,
							create_inherent_data_providers,
							block.origin.into(),
						)
						.await
						.map_err(|e| e.to_string())?;
					}

					let (_, inner_body) = new_block.deconstruct();
					block.body = Some(inner_body);
				}

				trace!(target: "aura", "Checked {:?}; importing.", pre_header);
				telemetry!(
					self.telemetry;
					CONSENSUS_TRACE;
					"aura.checked_and_importing";
					"pre_header" => ?pre_header,
				);

				// Look for an authorities-change log.
				let maybe_keys = pre_header
					.digest()
					.logs()
					.iter()
					.filter_map(|l| {
						l.try_to::<ConsensusLog<AuthorityId<P>>>(OpaqueDigestItemId::Consensus(
							&VOTE_ENGINE_ID,
						))
					})
					.find_map(|l| match l {
						ConsensusLog::AuthoritiesChange(a) =>
							Some(vec![(well_known_cache_keys::AUTHORITIES, a.encode())]),
						_ => None,
					});

				block.header = pre_header;
				block.post_digests.push(seal);
				block.fork_choice = Some(ForkChoiceStrategy::LongestChain);
				block.post_hash = Some(hash);

				Ok((block, maybe_keys))
			},
		}
	}
}

/// Should we check for equivocation of a block author?
#[derive(Debug, Clone, Copy)]
pub enum CheckForEquivocation {
	/// Yes, check for equivocation.
	///
	/// This is the default setting for this.
	Yes,
	/// No, don't check for equivocation.
	No,
}

impl Default for CheckForEquivocation {
	fn default() -> Self {
		Self::Yes
	}
}

/// Parameters of [`import_queue`].
pub struct ImportQueueParams<'a, Block, I, C, S, CAW, CIDP> {
	/// The block import to use.
	pub block_import: I,
	/// The justification import.
	pub justification_import: Option<BoxJustificationImport<Block>>,
	/// The client to interact with the chain.
	pub client: Arc<C>,
	/// Something that can create the inherent data providers.
	pub create_inherent_data_providers: CIDP,
	/// The spawner to spawn background tasks.
	pub spawner: &'a S,
	/// The prometheus registry.
	pub registry: Option<&'a Registry>,
	/// Can we author with the current node?
	pub can_author_with: CAW,
	/// Telemetry instance used to report telemetry metrics.
	pub telemetry: Option<TelemetryHandle>,
}

/// Start an import queue for the Aura consensus algorithm.
pub fn import_queue<'a, P, Block, I, C, S, CAW, CIDP>(
	ImportQueueParams {
		block_import,
		justification_import,
		client,
		create_inherent_data_providers,
		spawner,
		registry,
		can_author_with,
		telemetry,
	}: ImportQueueParams<'a, Block, I, C, S, CAW, CIDP>,
) -> Result<DefaultImportQueue<Block, C>, sp_consensus::Error>
where
	Block: BlockT,
	C::Api: BlockBuilderApi<Block> + VoteElectionApi<Block, AuthorityId<P>> + ApiExt<Block>,
	C: 'static
		+ ProvideRuntimeApi<Block>
		+ BlockOf
		+ Send
		+ Sync
		+ AuxStore
		+ UsageProvider<Block>
		+ HeaderBackend<Block>,
	I: BlockImport<Block, Error = ConsensusError, Transaction = sp_api::TransactionFor<C, Block>>
		+ Send
		+ Sync
		+ 'static,
	P: Pair + Send + Sync + 'static,
	P::Public: Clone + Eq + Send + Sync + Hash + Debug + Encode + Decode,
	P::Signature: Encode + Decode,
	S: sp_core::traits::SpawnEssentialNamed,
	CAW: CanAuthorWith<Block> + Send + Sync + 'static,
	CIDP: CreateInherentDataProviders<Block, ()> + Sync + Send + 'static,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send + Sync,
{
	let verifier = build_verifier::<P, _, _, _>(BuildVerifierParams {
		client,
		create_inherent_data_providers,
		can_author_with,
		telemetry,
	});

	Ok(BasicQueue::new(verifier, Box::new(block_import), justification_import, spawner, registry))
}

/// Parameters of [`build_verifier`].
pub struct BuildVerifierParams<C, CIDP, CAW> {
	/// The client to interact with the chain.
	pub client: Arc<C>,
	/// Something that can create the inherent data providers.
	pub create_inherent_data_providers: CIDP,
	/// Can we author with the current node?
	pub can_author_with: CAW,
	/// Telemetry instance used to report telemetry metrics.
	pub telemetry: Option<TelemetryHandle>,
}

/// Build the [`AuraVerifier`]
pub fn build_verifier<P, C, CIDP, CAW>(
	BuildVerifierParams {
		client,
		create_inherent_data_providers,
		can_author_with,
		telemetry,
	}: BuildVerifierParams<C, CIDP, CAW>,
) -> AuraVerifier<C, P, CAW, CIDP> {
	AuraVerifier::<_, P, _, _>::new(
		client,
		create_inherent_data_providers,
		can_author_with,
		telemetry,
	)
}
