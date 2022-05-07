// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Primitives for Aura.

#![cfg_attr(not(feature = "std"), no_std)]

pub use merlin::Transcript;

use codec::{Codec, Decode, Encode};
use sp_runtime::ConsensusEngineId;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use sp_keystore::vrf::{VRFTranscriptData, VRFTranscriptValue};

pub const VOTE_ENGINE_ID: ConsensusEngineId = *b"VOTE";
// pub const AURA_ENGINE_ID: ConsensusEngineId = [b'a', b'u', b'r', b'a'];

pub const VOTE_VRF_PREFIX: &[u8] = b"substrate-vote-vrf";

pub mod digests;
pub mod inherents;

pub mod sr25519 {
	mod app_sr25519 {
		use sp_application_crypto::{app_crypto, key_types::VOTE, sr25519};
		app_crypto!(sr25519, VOTE);
	}

	sp_application_crypto::with_pair! {
		/// An Aura authority keypair using S/R 25519 as its crypto.
		pub type AuthorityPair = app_sr25519::Pair;
	}

	/// An Aura authority signature using S/R 25519 as its crypto.
	pub type AuthoritySignature = app_sr25519::Signature;

	/// An Aura authority identifier using S/R 25519 as its crypto.
	pub type AuthorityId = app_sr25519::Public;
}

pub mod ed25519 {
	mod app_ed25519 {
		use sp_application_crypto::{app_crypto, ed25519, key_types::VOTE};
		app_crypto!(ed25519, VOTE);
	}

	sp_application_crypto::with_pair! {
		/// An Aura authority keypair using Ed25519 as its crypto.
		pub type AuthorityPair = app_ed25519::Pair;
	}

	/// An Aura authority signature using Ed25519 as its crypto.
	pub type AuthoritySignature = app_ed25519::Signature;

	/// An Aura authority identifier using Ed25519 as its crypto.
	pub type AuthorityId = app_ed25519::Public;
}

pub use sp_consensus_slots::Slot;

/// The `ConsensusEngineId` of AuRa.
// pub const AURA_ENGINE_ID: ConsensusEngineId = [b'a', b'u', b'r', b'a'];

/// The index of an authority.
pub type AuthorityIndex = u32;

/// An consensus log item for Aura.
#[derive(Decode, Encode)]
pub enum ConsensusLog<AuthorityId: Codec> {
	/// The authorities have changed.
	#[codec(index = 1)]
	AuthoritiesChange(Vec<AuthorityId>),
	/// Disable the authority with given index.
	#[codec(index = 2)]
	OnDisabled(AuthorityIndex),
}

sp_api::decl_runtime_apis! {
	/// API necessary for block authorship with aura.
	pub trait VoteElectionApi<AuthorityId: Codec> {
		/// Returns the slot duration for Aura.
		///
		/// Currently, only the value provided by this type at genesis will be used.
		fn slot_duration() -> SlotDuration;

		// Return the current set of authorities.
		fn authorities() -> Vec<AuthorityId>;
	}
}

/// Aura slot duration.
///
/// Internally stored as milliseconds.
#[derive(sp_runtime::RuntimeDebug, Encode, Decode, PartialEq, Clone, Copy)]
pub struct SlotDuration(u64);

impl SlotDuration {
	/// Initialize from the given milliseconds.
	pub fn from_millis(val: u64) -> Self {
		Self(val)
	}

	/// Returns the slot duration in milli seconds.
	pub fn get(&self) -> u64 {
		self.0
	}
}

#[cfg(feature = "std")]
impl sp_consensus::SlotData for SlotDuration {
	fn slot_duration(&self) -> std::time::Duration {
		std::time::Duration::from_millis(self.0)
	}
	const SLOT_KEY: &'static [u8] = b"vote_election_slot_duration";
}

/// Make a VRF transcript from given randomness, slot number and epoch.
pub fn make_transcript(msg: &Vec<u8>) -> Transcript {
	let mut transcript = Transcript::new(&VOTE_ENGINE_ID);
	transcript.append_message(b"chain hash", msg.as_slice());
	transcript
}

/// Make a VRF transcript data container
#[cfg(feature = "std")]
pub fn make_transcript_data(msg: &Vec<u8>) -> VRFTranscriptData {
	VRFTranscriptData {
		label: &VOTE_ENGINE_ID,
		items: vec![
			("chain hash", VRFTranscriptValue::Bytes(msg.clone())),
		],
	}
}
