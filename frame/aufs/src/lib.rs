#![cfg_attr(not(feature = "std"), no_std)]

// use safe_mix::TripletMix;

// use codec::Encode;
// use frame_support::traits::Randomness;
// use sp_runtime::traits::{Hash, Saturating};
use sp_std::{prelude::*};

// const RANDOM_MATERIAL_LEN: u32 = 81;

// fn block_number_to_index<T: Config>(block_number: T::BlockNumber) -> usize {
// 	// on_initialize is called on the first block after genesis
// 	let index = (block_number - 1u32.into()) % RANDOM_MATERIAL_LEN.into();
// 	index.try_into().ok().expect("Something % 81 is always smaller than usize; qed")
// }

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::storage]
	pub(super) type Value<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			if block_number == T::BlockNumber::from(10 u32){
				<Value<T>>::put(10);
			}

			// Self::update_storage();
			T::DbWeight::get().reads_writes(0, 0)
		}
	}

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self){
			<Value<T>>::put(4u32);
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn update_storage(){
		log::info!("evm-verkle update_storage");
	}
}