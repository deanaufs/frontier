#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{prelude::*, vec::Vec};
use codec::alloc::string::{String, ToString};
use scale_info::prelude::format;
use sp_core::{H160, H256, Bytes};
use sha3::{Digest, Keccak256};
use fp_evm::Log;
use hex_literal::hex;

use ethabi::{ Token, ParamType, Event, EventParam, RawLog };

pub use pallet::*;
const CID_LENGTH :usize = 32;
const AUFS_PREFIX: &str = "aufs://";

// event: "$SetURI(string,string,bytes32)"
const SET_URI_HASH: &[u8] = &hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");
const TRANSFER_HASH: &[u8] = &hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

// const SET_URI_STR : [u8;32] = hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");
// const SET_URI_HASH: [u8;32] = hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");
// const TRANSFER_HASH: [u8;32] = hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

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

	#[pallet::storage]
	#[pallet::getter(fn authorization)]
	pub type Authorization<T: Config> = StorageMap<_, Blake2_128Concat, String, T::BlockNumber, ValueQuery>;

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
				// if let Err(e) = Self::update_storage(){
				// 	log::info!("update uri failed: {}", e);
				// }
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

	impl<T: Config> pallet_evm::ACLManager for Pallet<T>{
		fn parse_log(source: H160, log: Log) {
			if let Some(event_hash) = log.topics.first(){
				match event_hash.as_bytes() {
					SET_URI_HASH =>{
						let _ = Self::set_uri(source, log.address, log.data);
					},
					TRANSFER_HASH =>{
						log::info!("event: transfer");
					},
					_ => {
						log::info!("undefined event: {:?}", event_hash);
					},
				};
			}

			// log::info!("Caller: {:?}", source);
			// log::info!(
			// 	target: "evm",
			// 	"Inserting log for {:?}, topics ({}) {:?}, data ({}): {:?}]",
			// 	log.address,
			// 	log.topics.len(),
			// 	log.topics,
			// 	log.data.len(),
			// 	log.data
			// );
		}
	}
}

impl<T: Config> Pallet<T> {
	fn set_uri(source: H160, contract_addr: H160, log_bytes: Vec<u8>)->Result<(), String>{
		// let log_bytes = log_data;
		// let log_bytes = [
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160,
		// 	8, 110, 81, 52, 145, 94, 30, 112, 191, 116, 252, 138, 98, 27, 112, 111,
		// 	169, 129, 103, 170, 231, 245, 2, 144, 240, 48, 74, 162, 99, 60, 44, 192,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12,
		// 	97, 117, 102, 115, 58, 47, 47, 97, 108, 105, 99, 101, 0, 0, 0, 0,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20,
		// 	47, 87, 101, 98, 51, 84, 117, 98, 101, 47, 109, 111, 118, 105, 101, 49,
		// 	46, 109, 112, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
		// ].to_vec();

		let (domain, path, cid_bytes) = Self::parse_set_uri_log(log_bytes)?;

		// domain: aufs://alice
		// path  : /Web3Youtube/movie1.map4
		// cid   : 0x086e5134915e1e70bf74fc8a621b706fa98167aae7f50290f0304aa2633c2cc0
		if Self::has_uri_map_write_permission(&source, &contract_addr, &domain){
			let key = [domain, path].join("");
			if cid_bytes == [0u8;32]{
				Uri::<T>::remove(key);
			}
			else{
				let value = cid_bytes;
				Uri::<T>::insert(key, value);
			}
		}
		// log::info!("save uri: {},{:?} ", key, value);
		// log::info!("event: set_uri");
		Ok(())
	}

	// domain: aufs://alice
	// path: /Web3Tube/dir1/movie.mp4

	// caller: 3637ccee721e0b3d9e3712c4c5dcdbc20b232bce
	// delegate: 3a6e65e7d282453e4f11161d2eba3856b4f69931
	fn has_uri_map_write_permission(caller: &H160, contract_addr: &H160, domain: &str)->bool{
		// let field = domain.to_string();
		// let caller_str = format!("{:?}", caller);
		// let delegate_str = format!("{:?}", delegate);

		// caller is the owner
		if domain.eq(&format!("{}{:?}", AUFS_PREFIX, caller)){
			return true;
		}

		// let path = String::from("/Web3Tube/dir1/movie.mp4");
		// let dir_vec = path.split("/").collect::<Vec<&str>>();

		// dir_vec.insert(0, "");

		// let now = <frame_system::Pallet<T>>::block_number();
		// let mut check_key = String::new();

		// for dir in dir_vec.iter(){
		// 	let height = Authorization::<T>::get(check_key);
		// 	if height >= now {
		// 		return true;
		// 	}
		// }

		// loop{
		// 	if true {
		// 		break;
		// 	}
		// 	check_key.push(new_path_item);
		// }

		return false;
	}

	fn parse_set_uri_log(log_bytes :Vec<u8>)->Result<(String,String,[u8;CID_LENGTH]), String>{
		let params = vec![
			EventParam{ name: "domain".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "path".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "cid".to_string(), kind: ParamType::FixedBytes(32), indexed: false, },
		];

		let event = Event {
			name: "$SetURI".to_string(),
			inputs: params,
			anonymous: false,
		};
		let ev_hash = event.signature();

		let raw_log = RawLog{
			topics: vec![ev_hash],
			data: log_bytes.to_vec(),
		};

		let log = event.parse_log(raw_log).map_err(|e|format!("parse log failed: {:?}", e))?;

		let domain = match &log.params.get(0).ok_or("get param domain failed")?.value{
			Token::String(s)=>{
				s.clone()
			}
			_ => {
				Err("parse $SetURI param 0 failed")?
			}
		};

		let path = match &log.params.get(1).ok_or("get param path failed")?.value{
			Token::String(s)=>{
				s.clone()
			}
			_ => {
				Err("parse $SetURI param 1 failed")?
			}
		};

		let cid_bytes = match &log.params.get(2).ok_or("get param cid failed")?.value{
			Token::FixedBytes(fixed_bytes)=>{
				if fixed_bytes.len() != CID_LENGTH{
					return Err("cid not 32 bytes")?;
				}
				let mut buf_bytes = [0u8;CID_LENGTH];
				buf_bytes.copy_from_slice(&fixed_bytes[0..CID_LENGTH]);
				buf_bytes
				// bytes.clone().as_chunks()
			}
			_ => {
				Err("parse $SetURI param 2 failed")?
			}
		};

		Ok((domain, path, cid_bytes))
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use log::*;
	use std::{io::Write, str::FromStr};
	use env_logger::Builder;
	use chrono::Local;

	fn init_logger() {
		Builder::new()
			.format(|buf, record| {
				writeln!(buf,
					"{} [{}] - {}",
					Local::now().format("%H:%M:%S"),
					record.level(),
					record.args()
				)
			})
			.filter(None, LevelFilter::Info)
			.init();
	}

	#[test]
	fn event_hash_test(){
		init_logger();
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
		log::info!("{:?}", ev_hash);
		if ev_hash.as_bytes() == hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2"){
			log::info!("same");
		}

		// log::info!("{:?}", ev_hash);
		let method_bytes = b"$SetURI(string,string,bytes32)";
		let method_hash = H256::from_slice(sha3::Keccak256::digest(method_bytes).as_slice());
		log::info!("{:?}", method_hash);
	}

	#[test]
	fn slice_test(){
		init_logger();
		// let caller = H160::from_str("3637CCeE721E0b3D9e3712C4c5DcDbC20b232bCE")
		// 	.expect("internal H160 is valid; qed");
		
		// // let s = caller.to_string();
		// // log::info!("{}", s);
		// // let s = caller.as_bytes().iter().map(|v|format!("{:02x}", v)).collect::<Vec<_>>().join("");
		// let s = format!("{:?}", caller);
		// log::info!("{}", s);

		// let domain =  "aufs://alice";
		// // let field = domain.to_string().split("//").collect().last();
		// // println!("{:?}", field);
		// if domain.to_string().ends_with("//alice"){
		// 	println!("yes");
		// }

		let path = String::from("/Web3Tube/dir1/movie.mp4");
		let mut item_vec = path.split("/").collect::<Vec<&str>>();
		item_vec.insert(0, "");
		let mut check_key = String::new();
		for item in item_vec.iter(){
			check_key.push_str(&format!("/{}", item));
			println!("{}", check_key);
		}
		// let a1: [u8;4] = [97;4];
		// let v = vec![1u8;4];
		// let s = "aaaa";
		// let a2 = s.as_bytes();
		// println!("{:?}", a2);

		// assert_eq!(a1, a2)

		// let a2: [u8] = [1,1,1,1];
		// let a2: [u8] = [1];

		// const FIX_ARRAY : [u8;32] = hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");
		// const fix_str: &str = "89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2";
		// let fix_bytes = hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");
		// let method_bytes = b"$SetURI(string,string,bytes32)";
		// let method_hash = H256::from_slice(sha3::Keccak256::digest(method_bytes).as_slice());
		// let hash_bytes = method_hash.as_bytes();

		// match hash_bytes{
		// 	fix_bytes=>{}
		// 	// hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2") => {
		// 	// 	println!("eq");
		// 	// },
		// 	_ => {}
		// }

		// if hash_bytes == fix_array{
		// 	// log::info!("same");
		// 	println!("same");
		// }

		// let set_uri_hash_bytes = hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");

		// match method_hash.as_bytes(){
		// 	set_uri_hash_bytes =>{
		// 		log::info!("");
		// 	}
		// 	_=>{

		// 	}
		// }
	}

	#[test]
	fn log_parse_test()->Result<(), String>{
		init_logger();
		let log_bytes = [
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160,
			8, 110, 81, 52, 145, 94, 30, 112, 191, 116, 252, 138, 98, 27, 112, 111,
			169, 129, 103, 170, 231, 245, 2, 144, 240, 48, 74, 162, 99, 60, 44, 192,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12,
			97, 117, 102, 115, 58, 47, 47, 97, 108, 105, 99, 101, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20,
			47, 87, 101, 98, 51, 84, 117, 98, 101, 47, 109, 111, 118, 105, 101, 49,
			46, 109, 112, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
		].to_vec();

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

		// let ev_hash = H256::from_slice(&hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2"));
		let ev_hash = event.signature();

		// let log_bytes = log.as_slice();
		let raw_log = RawLog{
			topics: vec![ev_hash],
			data: log_bytes,
		};

		let log = event.parse_log(raw_log).map_err(|e|format!("parse log failed: {:?}", e))?;

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
					return Err("cid not 32 bytes")?;
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

		log::info!("domain: {}", domain);
		log::info!("path: {}", path);
		log::info!("cid: {:?}", cid_bytes);

		Ok(())
	}
}