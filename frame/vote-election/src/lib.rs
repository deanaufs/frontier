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

//! # Aura Module
//!
//! - [`Config`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Aura module extends Aura consensus by managing offline reporting.
//!
//! ## Interface
//!
//! ### Public Functions
//!
//! - `slot_duration` - Determine the Aura slot-duration based on the Timestamp module
//!   configuration.
//!
//! ## Related Modules
//!
//! - [Timestamp](../pallet_timestamp/index.html): The Timestamp module is used in Aura to track
//! consensus rounds (via `slots`).

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	traits::{DisabledValidators, FindAuthor, Get, /*OnTimestampSet, */OneSessionHandler},
	BoundedSlice, ConsensusEngineId, Parameter, WeakBoundedVec,
};
use sp_consensus_vote_election::{AuthorityIndex, ConsensusLog, Slot, VOTE_ENGINE_ID};
use sp_runtime::{
	generic::DigestItem,
	traits::{IsMember, Member, /*Saturating*/},
	RuntimeAppPublic,
};
use sp_std::prelude::*;

pub mod migrations;
mod mock;
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: pallet_timestamp::Config + frame_system::Config {
		/// The identifier type for an authority.
		type AuthorityId: Member
			+ Parameter
			+ RuntimeAppPublic
			+ Default
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;
		/// The maximum number of authorities that the pallet can hold.
		type MaxAuthorities: Get<u32>;

		/// A way to check whether a given validator is disabled and should not be authoring blocks.
		/// Blocks authored by a disabled validator will lead to a panic as part of this module's
		/// initialization.
		type DisabledValidators: DisabledValidators;
	}

	#[pallet::pallet]
	#[pallet::generate_storage_info]
	pub struct Pallet<T>(sp_std::marker::PhantomData<T>);

	/// The current authority set.
	#[pallet::storage]
	#[pallet::getter(fn authorities)]
	pub(super) type Authorities<T: Config> =
		StorageValue<_, WeakBoundedVec<T::AuthorityId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub authorities: Vec<T::AuthorityId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { authorities: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_authorities(&self.authorities);
		}
	}
}

impl<T: Config> Pallet<T> {
	fn change_authorities(new: WeakBoundedVec<T::AuthorityId, T::MaxAuthorities>) {
		<Authorities<T>>::put(&new);

		let log = DigestItem::Consensus(
			VOTE_ENGINE_ID,
			ConsensusLog::AuthoritiesChange(new.into_inner()).encode(),
		);
		<frame_system::Pallet<T>>::deposit_log(log.into());
	}

	fn initialize_authorities(authorities: &[T::AuthorityId]) {
		if !authorities.is_empty() {
			assert!(<Authorities<T>>::get().is_empty(), "Authorities are already initialized!");
			let bounded = <BoundedSlice<'_, _, T::MaxAuthorities>>::try_from(authorities)
				.expect("Initial authority set must be less than T::MaxAuthorities");
			<Authorities<T>>::put(bounded);
		}
	}
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
	type Public = T::AuthorityId;
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
	type Key = T::AuthorityId;

	fn on_genesis_session<'a, I: 'a>(validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{
		let authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
		Self::initialize_authorities(&authorities);
	}

	fn on_new_session<'a, I: 'a>(changed: bool, validators: I, _queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{
		// instant changes
		if changed {
			let next_authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
			let last_authorities = Self::authorities();
			if last_authorities != next_authorities {
				let bounded = <WeakBoundedVec<_, T::MaxAuthorities>>::force_from(
					next_authorities,
					Some("AuRa new session"),
				);
				Self::change_authorities(bounded);
			}
		}
	}

	fn on_disabled(i: usize) {
	// fn on_disabled(i: u32) {
		let log = DigestItem::Consensus(
			VOTE_ENGINE_ID,
			ConsensusLog::<T::AuthorityId>::OnDisabled(i as AuthorityIndex).encode(),
		);

		<frame_system::Pallet<T>>::deposit_log(log.into());
	}
}

impl<T: Config> FindAuthor<u32> for Pallet<T> {
	fn find_author<'a, I>(digests: I) -> Option<u32>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		for (id, mut data) in digests.into_iter() {
			if id == VOTE_ENGINE_ID {
				let slot = Slot::decode(&mut data).ok()?;
				let author_index = *slot % Self::authorities().len() as u64;
				return Some(author_index as u32)
			}
		}

		None
	}
}

/// We can not implement `FindAuthor` twice, because the compiler does not know if
/// `u32 == T::AuthorityId` and thus, prevents us to implement the trait twice.
#[doc(hidden)]
pub struct FindAccountFromAuthorIndex<T, Inner>(sp_std::marker::PhantomData<(T, Inner)>);

impl<T: Config, Inner: FindAuthor<u32>> FindAuthor<T::AuthorityId>
	for FindAccountFromAuthorIndex<T, Inner>
{
	fn find_author<'a, I>(digests: I) -> Option<T::AuthorityId>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		let i = Inner::find_author(digests)?;

		let validators = <Pallet<T>>::authorities();
		validators.get(i as usize).map(|k| k.clone())
	}
}

/// Find the authority ID of the Aura authority who authored the current block.
pub type AuraAuthorId<T> = FindAccountFromAuthorIndex<T, Pallet<T>>;

impl<T: Config> IsMember<T::AuthorityId> for Pallet<T> {
	fn is_member(authority_id: &T::AuthorityId) -> bool {
		Self::authorities().iter().any(|id| id == authority_id)
	}
}
