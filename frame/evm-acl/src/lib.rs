#![cfg_attr(not(feature = "std"), no_std)]

// use safe_mix::TripletMix;

// use codec::Encode;
// use frame_support::traits::Randomness;
// use sp_runtime::traits::{Hash, Saturating};
use sp_std::{prelude::*, vec::Vec};
// use sp_std::borrow::ToOwned;
// use sp_std::alloc;
use codec::alloc::string::{String, ToString};
// use scale_info::prelude::string::String;
use scale_info::prelude::format;

use ethabi::{ Token, ParamType, Event, EventParam, RawLog };

pub use pallet::*;
const CID_LENGTH :usize = 32;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::storage]
	pub type Value<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn uri)]
	pub type Uri<T: Config> = StorageMap<_, Blake2_128Concat, String, [u8;CID_LENGTH], ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn authorization)]
	// pub type Authorization<T: Config> = StorageMap<_, Blake2_128Concat, String, T::BlockNumber, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn delegate)]
	// pub type Delegate<T: Config> = StorageMap<_, Blake2_128Concat, String, Vec<T::AccountId>, ValueQuery>;

	// #[pallet::call]
	// impl<T: Config> Pallet<T>{
	// 	#[pallet::weight(50_000_000)]
	// 	pub fn set_uri(origin: OriginFor<T>, contract_addr: Option<T::AccountId>, logs: Logs)->DispatchResult{
	// 		log::info!("set_uri(), {:?}, {:?}, {:?}", ori);
	// 		Ok(())
	// 	}
	// }

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			if block_number == T::BlockNumber::from(3u32){
				if let Err(e) = Self::update_storage(){
					log::info!("update uri failed: {}", e);
				}
				<Value<T>>::put(77);
			}

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
	pub fn update_storage()->Result<(), String>{
		let params = vec![
			EventParam{
				name: "domain".to_string(),
				kind: ParamType::String,
				indexed: false,
			},
			EventParam{
				name: "path".to_string(),
				kind: ParamType::String,
				indexed: false,
			},
			EventParam{
				name: "cid".to_string(),
				kind: ParamType::FixedBytes(32),
				indexed: false,
			},
		];

		let event = Event {
			name: "$SetURI".to_string(),
			inputs: params,
			anonymous: false,
		};

		let ev_hash = event.signature();
		// println!("{:?}", ev_hash);

		let log_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 8, 110, 81, 52, 145, 94, 30, 112, 191, 116, 252, 138, 98, 27, 112, 111, 169, 129, 103, 170, 231, 245, 2, 144, 240, 48, 74, 162, 99, 60, 44, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 97, 117, 102, 115, 58, 47, 47, 97, 108, 105, 99, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 47, 87, 101, 98, 51, 84, 117, 98, 101, 47, 109, 111, 118, 105, 101, 49, 46, 109, 112, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
		// let topic_value = H256::from_slice(&hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2"));
		let raw_log = RawLog{
			topics: vec![ev_hash],
			data: log_bytes.to_vec(),
		};

		// let r = event.parse_log(raw_log);
		// if let Ok(l) = r{
		// 	for (i, param) in l.params.iter().enumerate(){
		// 		match &param.value{
		// 			Token::String(s)=>{
		// 				log::info!("{}: {}", i, s);
		// 			},
		// 			Token::FixedBytes(bytes_vec)=>{
		// 				let out_str = bytes_vec.iter().map(|v|format!("{:02x}", v)).collect::<Vec<_>>().join("");
		// 				log::info!("{}: 0x{}", i, out_str);
		// 			},
		// 			_ =>{},
		// 		}
		// 	}
		// }

		let log = event.parse_log(raw_log).map_err(|e|format!("parse log failed: {:?}", e))?;
		// let domain = log.params.get(0).ok_or("get name failed")?;
		// let path = log.params.get(1).ok_or("get name failed")?;

		let domain = match &log.params.get(0).ok_or("get param domain failed")?.value{
			Token::String(s)=>{
				s.clone()
			}
			_ => {
				Err("parse log param failed")?
			}
		};

		let path = match &log.params.get(1).ok_or("get param path failed")?.value{
			Token::String(s)=>{
				s.clone()
			}
			_ => {
				Err("parse log param failed")?
			}
		};

		let cid_bytes = match &log.params.get(2).ok_or("get param cid failed")?.value{
			Token::FixedBytes(fixed_bytes)=>{
				if fixed_bytes.len() != CID_LENGTH{
					return Err("cid not 32 bytes");
				}
				let mut buf_bytes = [0u8;CID_LENGTH];
				buf_bytes.copy_from_slice(&fixed_bytes[0..CID_LENGTH]);
				buf_bytes
				// bytes.clone().as_chunks()
			}
			_ => {
				Err("parse log param failed")?
			}
		};

		// aufs://alice
		// /Web3Youtube/movie1.map4
		// 0x086e5134915e1e70bf74fc8a621b706fa98167aae7f50290f0304aa2633c2cc0
		// let name = params.as_ref().get(0).ok_or("get name failed")?;


		let key = [domain, path].join("");
		let value = cid_bytes;
		log::info!("save uri: {},{:?} ", key, value);
		Uri::<T>::insert(key, value);

		// log::info!("{:?}", r);
		// log::info!("aufs update_storage");
		Ok(())
	}
}