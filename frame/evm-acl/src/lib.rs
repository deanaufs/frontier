#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{prelude::*, vec::Vec};
use codec::alloc::string::{String, ToString};
use scale_info::prelude::format;
use sp_core::{H160, H256, Bytes, U256};
use sha3::{Digest, Keccak256};
use fp_evm::Log;
use hex_literal::hex;

use ethabi::{ Token, ParamType, Event, EventParam, RawLog };

pub use pallet::*;
const CID_LENGTH :usize = 32;
const AUFS_PREFIX: &str = "aufs://";
const KEY_DELEGATED: &str = "_delegated";

// event: "$SetURI(string,string,bytes32)"
const SET_URI_HASH: &[u8] = 
	&hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2");

// event: $SetAuthorization(string,uint8,address,u32)
const SET_AUTHORIZATION_HASH: &[u8] = 
	&hex!("ba3093b03ca4755ef7c520662e49261a353f31a25288a73f6aac910b25697c81");

// event: $SetDelegate(string,address,uint8)
const SET_DELEGATE: &[u8] = 
	&hex!("784d07d8be5f051a3f05b02a551e41c5dd705fcf3ac4eae3a9b48d9905491939");

const TRANSFER_HASH: &[u8] = 
	&hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

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
	pub type Uri<T: Config> = StorageMap<_, Blake2_128Concat, String, [u8;CID_LENGTH]>;

	#[pallet::storage]
	pub type Authorization<T: Config> = StorageMap<_, Blake2_128Concat, String, T::BlockNumber>;
	// pub type Authorization<T: Config> = StorageMap<_, Blake2_128Concat, String, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	pub type Delegate<T: Config> = StorageMap<_, Blake2_128Concat, String, Vec<H160>>;

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
						if let Err(e) = Self::set_uri(source, log.address, log.data){
							log::info!("set uri failed: {}", e);
						}
					},
					SET_AUTHORIZATION_HASH => {
						if let Err(e) = Self::set_authorization(source, log.address, log.data){
							log::info!("set authorization failed: {}", e);
						}
					},
					SET_DELEGATE => {
						if let Err(e) = Self::set_delegate(source, log.address, log.data){
							log::info!("set delegate failed: {}", e);
						}
					},
					TRANSFER_HASH =>{
						// if let Err(e) = Self::set_authorization(source, log.address, log.data){
						// 	log::info!("set authorization failed: {}", e);
						// }
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
		if Self::check_uri_set_permission(&source, &contract_addr, &domain, &path){
			let key = [domain, path].join("");
			if cid_bytes == [0u8;32]{
				Uri::<T>::remove(key);
			}
			else{
				let value = cid_bytes;
				Uri::<T>::insert(key, value);
			}
		}
		Ok(())
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

	/// domain: "aufs://alice"
	/// path: "/Web3Tube/dir1/movie.mp4"
	/// caller: 0x3637ccee721e0b3d9e3712c4c5dcdbc20b232bce
	/// contract_addr: 0x3a6e65e7d282453e4f11161d2eba3856b4f69931
	fn check_uri_set_permission(caller: &H160, contract_addr: &H160, domain: &str, path: &str)->bool{
		// check if caller is the owner
		let owner_domain = format!("{}{:?}", AUFS_PREFIX, caller);
		if domain.eq(&owner_domain){
			return true;
		}

		// check if contract has write authorization
		let contract_str = format!("{:?}", contract_addr);
		let path = String::from(path);
		let sub_dir_vec = path.split("/").collect::<Vec<&str>>();

		let now = <frame_system::Pallet<T>>::block_number();

		let mut check_dir = String::new();
		for (i, sub_dir) in sub_dir_vec.iter().enumerate(){
			if i!= 0{
				check_dir.push_str(&format!("/{}", sub_dir));
			}
			let check_key = format!("{domain}{check_dir}@write#{contract_str}");
			// log::info!("{check_key}");

			if let Some(height) = Authorization::<T>::get(&check_key){
				if height > now{
					return true;
				}
			}
		}
		return false;
	}
}

impl<T: Config> Pallet<T> {
	fn set_authorization(source: H160, contract_addr: H160, log_bytes: Vec<u8>)->Result<(), String>{
		let (domain, path, rw_type, target_addr, set_height) = Self::parse_authorization_log(log_bytes)?;

		let now = <frame_system::Pallet<T>>::block_number();

		// let source = H160::from(hex!("dd31c61141abd04a99fd6822c8558d43593c715f"));
		// let domain = "aufs://alice";
		// let path = "/Web3Tube/dir1/movie.mp4";
		// let rw_type = 0u8;
		// let contract_addr = H160::from(hex!("d43593c715fdd31c61141abd04a99fd6822c8558"));

		// let target_addr = H160::from(hex!("04a99fd6822c8d43593c715fdd31c61141abd558"));
		// let set_height = T::BlockNumber::from(100u32);

		if Self::check_authorization_set_permission(&source, &contract_addr, &domain, &path, rw_type){
			let rw_type_str = {
				if rw_type == 0{ String::from("read") }
				else if rw_type == 1{ String::from("write") }
				else{
					return Err(format!("Set authorization type value err, expect: 0,1, recv: {}", rw_type));
				}
			};
			let key = format!("{domain}{path}@{rw_type_str}#{:?}", target_addr);

			if (set_height > now) || ( set_height == T::BlockNumber::from(0u32) ){
				Authorization::<T>::insert(key, set_height );
			}
			else{
				Authorization::<T>::remove(key);
			}
		}

		Ok(())
	}

	fn check_authorization_set_permission(caller: &H160, contract_addr: &H160, domain: &str, path: &str, rw_type: u8)->bool{

		let owner_domain = format!("{}{:?}", AUFS_PREFIX, caller);
		if owner_domain.eq(domain){
			return true;
		}
		let path = String::from(path);

		let sub_path_vec = path.split("/").collect::<Vec<&str>>();
		let rw_type_str = {
			if rw_type == 0u8{ String::from("read") }
			else if rw_type == 1u8 {String::from("write")}
			else{
				return false;
			}
		};


		let mut check_path = String::new();
		for (i, sub_path) in sub_path_vec.iter().enumerate(){
			if i != 0{
				check_path.push_str(&format!("/{}", sub_path));
			}
			let check_key = format!("{}{}@{KEY_DELEGATED}#{}", domain, check_path, rw_type_str);
			log::info!("{}", check_key);
			if let Some(addr_vec) = <Delegate<T>>::get(check_key){
				if addr_vec.contains(contract_addr){
					return true;
				}
			}
		}

		return false;
	}

	// $SetAuthorization(string,string,uint8,address,u32)
	fn parse_authorization_log(log_bytes: Vec<u8>)->Result<(String, String, u8, H160, T::BlockNumber), String>{
		let params = vec![
			EventParam{ name: "domain".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "path".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "rw_type".to_string(), kind: ParamType::FixedBytes(1), indexed: false, },
			EventParam{ name: "target_address".to_string(), kind: ParamType::Address, indexed: false, },
			EventParam{ name: "height".to_string(), kind: ParamType::Uint(1), indexed: false, },
		];

		let event = Event {
			name: "$SetAuthorization".to_string(),
			inputs: params,
			anonymous: false,
		};
		let ev_hash = event.signature();

		let raw_log = RawLog{
			topics: vec![ev_hash],
			data: log_bytes.to_vec(),
		};

		let log = event.parse_log(raw_log).map_err(|e|format!("parse $SetAuthorization log failed: {:?}", e))?;

		let domain = match &log.params.get(0).ok_or("get $SetAuthorization param 0 failed")?.value{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse $SetAuthorization param 0 failed")?  }
		};
		let path = match &log.params.get(1).ok_or("get $SetAuthorization param 1 failed")?.value{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse $SetAuthorization param 1 failed")?  }
		};
		let rw_type = match &log.params.get(2).ok_or("get $SetAuthorization param 2 failed")?.value{
			Token::FixedBytes(rw_bytes)=>{ 
				if rw_bytes.len()!=1{
					return Err("rw type is not 1 byte")?;
				}
				if rw_bytes[0] == 0 || rw_bytes[0] ==1{
					rw_bytes[0].clone()
				}
				else{
					return Err("rw type value not [0,1]")?;
				}
			}
			_ => { Err("parse $SetAuthorization param 2 failed")?  }
		};
		let target_addr = match &log.params.get(3).ok_or("get $SetAuthorization param 3 failed")?.value{
			Token::Address(addr)=>{ addr.clone() }
			_ => { Err("parse $SetAuthorization param 3 failed")?  }
		};
		let height = match &log.params.get(4).ok_or("get $SetAuthorization param 4 failed")?.value{
			Token::Uint(n)=>{ T::BlockNumber::from(n.as_u32()) }
			_ => { Err("parse $SetAuthorization param 4 failed")?  }
		};

		Ok((domain, path, rw_type, target_addr, height))
	}
}

impl<T: Config> Pallet<T>{
	fn set_delegate(source: H160, _: H160, log: Vec<u8>)->Result<(), String>{
		let (domain, path, set_type, target_addr) = Self::parse_delegate_log(log)?;

		if Self::check_delegate_set_permission(&source, &domain){
			// "aufs://alice/dir1@_delegated#read"
			match set_type{
				1|2 => {
					let rw_str = {
						if set_type == 1{ String::from("read") }
						else{ String::from("write") }
					};

					let key = format!("{}{}@{KEY_DELEGATED}#{}", domain, path, rw_str); 
					Delegate::<T>::mutate(key, |addr_vec| {
						if let Some(addr_vec) =  addr_vec.as_mut(){
							if !addr_vec.contains(&target_addr){
								addr_vec.push(target_addr.clone());
							}
						}
					});
				},
				0 =>{
					let read_key = format!("{}{}@{KEY_DELEGATED}#read", domain, path); 
					Delegate::<T>::mutate(read_key, |addr_vec|{
						if let Some(addr_vec) =  addr_vec.as_mut(){
							let index = addr_vec.iter().position(|&v|v==target_addr);
							if let Some(index) = index{
								addr_vec.remove(index);
							}
						}
					});
					let write_key = format!("{}{}@{KEY_DELEGATED}#write", domain, path); 
					Delegate::<T>::mutate(write_key, |addr_vec|{
						if let Some(addr_vec) =  addr_vec.as_mut(){
							let index = addr_vec.iter().position(|&v|v==target_addr);
							if let Some(index) = index{
								addr_vec.remove(index);
							}
						}
					});
				}
				_ => {
					Err("$SetDelegate set_value err")?
				}
			}
		}
		Ok(())
	}

	fn check_delegate_set_permission(caller: &H160, domain: &str)->bool{
		let owner_domain = format!("{}{:?}", AUFS_PREFIX, caller);
		if owner_domain.eq(domain){
			return true;
		}

		return false;
	}

	fn parse_delegate_log(log: Vec<u8>)->Result<(String, String, u8, H160), String>{
		let params = vec![
			EventParam{ name: "domain".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "path".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "set_type".to_string(), kind: ParamType::FixedBytes(1), indexed: false, },
			EventParam{ name: "target_address".to_string(), kind: ParamType::Address, indexed: false, },
		];

		let event = Event {
			name: "$SetDelegate".to_string(),
			inputs: params,
			anonymous: false,
		};
		let ev_hash = event.signature();

		let raw_log = RawLog{
			topics: vec![ev_hash],
			data: log,
		};

		let log = event.parse_log(raw_log).map_err(|e|format!("parse $SetDelegate log failed: {:?}", e))?;

		let domain = match &log.params.get(0).ok_or("get $SetDelegate param 0 failed")?.value{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse $SetDelegate param 0 failed")?  }
		};
		let path = match &log.params.get(1).ok_or("get $SetDelegate param 1 failed")?.value{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse $SetDelegate param 1 failed")?  }
		};
		let set_type = match &log.params.get(2).ok_or("get $SetDelegate param 2 failed")?.value{
			Token::FixedBytes(rw_bytes)=>{ 
				if rw_bytes.len()!=1{
					return Err("set_value is not 1 byte")?;
				}
				match rw_bytes[0]{
					0|1|2 =>{ rw_bytes[0].clone() }
					_ => { return Err("set type value not in [0,1,2]")?; }
				}
			}
			_ => { Err("parse $SetDelegate param 2 failed")?  }
		};
		let target_addr = match &log.params.get(3).ok_or("get $SetDelegate param 3 failed")?.value{
			Token::Address(addr)=>{ addr.clone() }
			_ => { Err("parse $SetDelegate param 3 failed")?  }
		};

		Ok((domain, path, set_type, target_addr))
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use log::*;
	use std::{io::Write,};
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