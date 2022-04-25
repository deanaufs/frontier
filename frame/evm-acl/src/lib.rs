#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{prelude::*, vec::Vec};
use codec::alloc::string::{String};
use scale_info::prelude::format;
use sp_core::{H160, U256};
use fp_evm::Log;
use hex_literal::hex;
use frame_support::traits::GenesisBuild;

use ethabi::{Token, ParamType};

pub use pallet::*;
const CID_LENGTH :usize = 32;
const AUFS_PREFIX: &str = "aufs://";

const STR_DELEGATED: &str = "_delegated";
const STR_READ: &str = "read";
const STR_WRITE: &str = "write";

// const DELETE_VALUE: u8 = 0u8;
const SET_READ :u8 = 1u8;
const SET_WRITE :u8 = 2u8;

const DELETE_AUTHORIZATION_BLOCK_HEIGHT: u32 = 0u32;
const DELETE_URI_BYTES: [u8;32] = [0u8;32];

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
	#[pallet::getter(fn value)]
	pub type Value<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn uri)]
	pub type Uri<T: Config> = StorageMap<_, Blake2_128Concat, String, [u8;CID_LENGTH]>;

	#[pallet::storage]
	#[pallet::getter(fn authorization)]
	pub type Authorization<T: Config> = StorageMap<_, Blake2_128Concat, String, T::BlockNumber>;
	// pub type Authorization<T: Config> = StorageMap<_, Blake2_128Concat, String, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn delegate)]
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
	// #[derive(Default)]
	pub struct GenesisConfig<T: Config> {
		_marker: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self){
			<Value<T>>::put(4u32);
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				_marker: PhantomData,
			}
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
						if let Err(e) = Self::set_delegate(source, log.data){
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

		let (domain, path, cid_bytes) = Self::parse_set_uri_log(log_bytes)?;
		// log::info!("{}, {}, {:?}", domain, path, cid_bytes);

		// domain: aufs://0x0000000000000000000000000000000000000001
		// path  : /Web3Youtube/movie1.map4
		// cid   : 0x086e5134915e1e70bf74fc8a621b706fa98167aae7f50290f0304aa2633c2cc0
		if Self::check_uri_set_permission(&source, &contract_addr, &domain, &path){
			let key = [domain, path].join("");
			if cid_bytes == DELETE_URI_BYTES {
				Uri::<T>::remove(key);
				return Ok(());
			}
			else{
				let value = cid_bytes;
				Uri::<T>::insert(key, value);
				return Ok(());
			}
		}
		else{
			Err("No set permission")?
		}
	}

	fn parse_set_uri_log(log_bytes :Vec<u8>)->Result<(String,String,[u8;CID_LENGTH]), String>{
		let params_type = [
			ParamType::String,
			ParamType::String,
			ParamType::FixedBytes(32),
		];
		let params = ethabi::decode(&params_type, &log_bytes).map_err(|e|format!("decode failed: {}", e))?;

		let domain = match &params.get(0).ok_or("get param domain failed")?{
			Token::String(s)=>{
				s.clone()
			}
			_ => {
				Err("parse param 0 failed")?
			}
		};

		let path = match &params.get(1).ok_or("get param path failed")?{
			Token::String(s)=>{
				s.clone()
			}
			_ => {
				Err("parse param 1 failed")?
			}
		};

		let cid_bytes = match &params.get(2).ok_or("get param cid failed")?{
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
				Err("parse param 2 failed")?
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
			let check_key = format!("{domain}{check_dir}@{STR_WRITE}#{contract_str}");
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
	fn set_authorization(caller: H160, contract_addr: H160, log_bytes: Vec<u8>)->Result<(), String>{
		let (domain, path, rw_type, target_addr, set_height) = Self::parse_authorization_log(log_bytes)?;

		let now = <frame_system::Pallet<T>>::block_number();

		if Self::check_authorization_set_permission(&caller, &contract_addr, &domain, &path, rw_type, &target_addr){
			let rw_type_str = {
				if rw_type == SET_READ { String::from(STR_READ) }
				else if rw_type == SET_WRITE { String::from(STR_WRITE) }
				else{
					return Err(format!("Set authorization type value err, expect: 0,1, recv: {}", rw_type));
				}
			};
			let key = format!("{domain}{path}@{rw_type_str}#{:?}", target_addr);

			if (set_height > now) || ( set_height == T::BlockNumber::from(DELETE_AUTHORIZATION_BLOCK_HEIGHT) ){
				Authorization::<T>::insert(key, set_height );
			}
			else{
				Authorization::<T>::remove(key);
			}
		}
		Ok(())
	}

	fn check_authorization_set_permission(
		caller: &H160,
		contract_addr: &H160,
		domain: &str,
		path: &str,
		rw_type: u8,
		target_addr: &H160,
	)->bool{
		// set authorization by owner
		let owner_domain = format!("{}{:?}", AUFS_PREFIX, caller);
		if owner_domain.eq(domain){
			return true;
		}

		// set authorization by contract
		if caller != target_addr{
			return false;
		}

		let path = String::from(path);
		let sub_path_vec = path.split("/").collect::<Vec<&str>>();

		if rw_type == SET_READ{
			// check delegate read/write
			let mut check_path = String::new();
			for (i, sub_path) in sub_path_vec.iter().enumerate(){
				if i != 0{
					check_path.push_str(&format!("/{}", sub_path));
				}
				// log::info!("{}", check_path);

				// aufs://0x0000000000000000000000000000000000000001/Web3Tube@_delegated#read
				let check_read_key = format!("{}{}@{STR_DELEGATED}#{}", domain, check_path, STR_READ);
				// log::info!("{}", check_read_key);
				if let Some(addr_vec) = <Delegate<T>>::get(check_read_key){
					if addr_vec.contains(contract_addr){
						return true;
					}
				}

				let check_write_key = format!("{}{}@{STR_DELEGATED}#{}", domain, check_path, STR_WRITE);
				// log::info!("{}", check_write_key);
				if let Some(addr_vec) = <Delegate<T>>::get(check_write_key){
					if addr_vec.contains(contract_addr){
						return true;
					}
				}
			}
		}
		else if rw_type == SET_WRITE{
			// only check delegate write
			let mut check_path = String::new();
			for (i, sub_path) in sub_path_vec.iter().enumerate(){
				if i != 0{
					check_path.push_str(&format!("/{}", sub_path));
				}

				let check_write_key = format!("{}{}@{STR_DELEGATED}#{}", domain, check_path, STR_WRITE);
				if let Some(addr_vec) = <Delegate<T>>::get(check_write_key){
					if addr_vec.contains(contract_addr){
						return true;
					}
				}
			}
		}

		return false;
	}

	// $SetAuthorization(string,string,uint8,address,u32)
	fn parse_authorization_log(log_bytes: Vec<u8>)->Result<(String, String, u8, H160, T::BlockNumber), String>{
		let params_type = [
			ParamType::String,
			ParamType::String,
			ParamType::Uint(32),
			ParamType::Address,
			ParamType::Uint(32),
		];
		let params = ethabi::decode(&params_type, &log_bytes).map_err(|e|format!("pasre log failed: {}", e))?;

		let domain = match &params.get(0).ok_or("get param domain failed")?{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse param domain failed")?  }
		};
		let path = match &params.get(1).ok_or("get param path failed")?{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse param path failed")?  }
		};
		let rw_type = match &params.get(2).ok_or("get param rw_type failed")?{
			Token::Uint(n)=>{
				if n == &U256::from(SET_READ){
					SET_READ
				}
				else if n == &U256::from(SET_WRITE){
					SET_WRITE
				}
				else{
					Err("unexpect rw_type value")?
				}
			},
			_ => {
				Err("parse param rw_type failed")?
			}
		};
		let target_addr = match &params.get(3).ok_or("get param target_address failed")?{
			Token::Address(addr)=>{ addr.clone() }
			_ => { Err("parse param target_address failed")?  }
		};
		let height = match &params.get(4).ok_or("get param height failed")?{
			Token::Uint(n)=>{ T::BlockNumber::from(n.as_u32()) }
			_ => { Err("parse param height failed")?  }
		};

		Ok((domain, path, rw_type, target_addr, height))
	}

	fn _check_authorization_set_permission_v1(caller: &H160, contract_addr: &H160, domain: &str, path: &str, rw_type: u8)->bool{
		let owner_domain = format!("{}{:?}", AUFS_PREFIX, caller);
		if owner_domain.eq(domain){
			return true;
		}
		let path = String::from(path);

		let sub_path_vec = path.split("/").collect::<Vec<&str>>();
		let rw_type_str = {
			if rw_type == SET_READ { 
				String::from(STR_READ) 
			}
			else if rw_type == SET_WRITE {
				String::from(STR_WRITE)
			}
			else{
				return false;
			}
		};

		let mut check_path = String::new();
		for (i, sub_path) in sub_path_vec.iter().enumerate(){
			if i != 0{
				check_path.push_str(&format!("/{}", sub_path));
			}
			let check_key = format!("{}{}@{STR_DELEGATED}#{}", domain, check_path, rw_type_str);
			// log::info!("{}", check_key);
			if let Some(addr_vec) = <Delegate<T>>::get(check_key){
				if addr_vec.contains(contract_addr){
					return true;
				}
			}
		}

		return false;
	}
}

impl<T: Config> Pallet<T>{
	fn set_delegate(source: H160, log: Vec<u8>)->Result<(), String>{
		let (domain, path, set_type, is_remove, target_addr) = Self::parse_delegate_log(log)?;

		if Self::check_delegate_set_permission(&source, &domain){
			// "aufs://alice/dir1@_delegated#read"
			if set_type == SET_READ{
				let key = format!("{}{}@{STR_DELEGATED}#{}", domain, path, STR_READ); 
				if is_remove{
					Self::remove_delegate_item(&key, &target_addr);
				}
				else{
					Self::append_delegate_item(&key, &target_addr);
				}
			}
			if set_type == SET_WRITE{
				let key = format!("{}{}@{STR_DELEGATED}#{}", domain, path, STR_WRITE); 
				if is_remove{
					Self::remove_delegate_item(&key, &target_addr);
				}
				else{
					Self::append_delegate_item(&key, &target_addr);
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

	fn parse_delegate_log(log_bytes: Vec<u8>)->Result<(String, String, u8, bool, H160), String>{
		let params_type = [
			ParamType::String,
			ParamType::String,
			ParamType::Uint(32),
			ParamType::Bool,
			ParamType::Address,
		];
		let params = ethabi::decode(&params_type, &log_bytes).map_err(|e|format!("parse log failed: {}", e))?;

		let domain = match &params.get(0).ok_or("get param domain failed")?{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse param domain failed")?  }
		};
		let path = match &params.get(1).ok_or("get param path failed")?{
			Token::String(s)=>{ s.clone() }
			_ => { Err("parse param path failed")?  }
		};
		let set_type = match &params.get(2).ok_or("get param set_type failed")?{
			Token::Uint(n)=>{
				if n == &U256::from(SET_READ){
					SET_READ
				}
				else if n == &U256::from(SET_WRITE){
					SET_WRITE
				}
				else{
					Err("Unexpect set_type value")?
				}
			},
			_ => {
				Err("parse param set_type failed")?
			}
		};
		let is_remove = match &params.get(3).ok_or("get param is_remove failed")?{
			Token::Bool(b)=>{
				b.clone()
			}
			_ => {
				Err("parse param is_remove failed")?
			}
		};
		let target_addr = match &params.get(4).ok_or("get param target_address failed")?{
			Token::Address(addr)=>{ addr.clone() }
			_ => { Err("parse param target_address failed")?  }
		};

		Ok((domain, path, set_type, is_remove, target_addr))
	}

	fn remove_delegate_item(key: &str, target_addr: &H160){
		let mut delete_key = false;
		// let read_key = format!("{}{}@{STR_DELEGATED}#{}", domain, path, STR_READ); 
		Delegate::<T>::mutate(key, |addr_vec|{
			if let Some(addr_vec) =  addr_vec.as_mut(){
				let index = addr_vec.iter().position(|v|v==target_addr);
				if let Some(index) = index{
					addr_vec.remove(index);
					if addr_vec.len() == 0{
						delete_key = true;
					}
				}
			}
		});
		if delete_key{
			Delegate::<T>::remove(key);
		}
	}

	fn append_delegate_item(key: &str, target_addr: &H160){
		if Delegate::<T>::get(key).is_none(){
			Delegate::<T>::insert(key, vec![target_addr.clone()]);
		}
		else{
			Delegate::<T>::mutate(key, |addr_vec| {
				if let Some(addr_vec) =  addr_vec.as_mut(){
					if !addr_vec.contains(target_addr){
						addr_vec.push(target_addr.clone());
					}
				}
			});
		}
	}

}

#[cfg(test)]
mod test2{
	use super::*;

	use log::*;
	use std::{io::Write,};
	use sp_core::H256;
	use env_logger::Builder;
	use chrono::Local;
	use ethabi::EventParam;
	// use ethabi::encode;

	use crate as pallet_evm_acl;
	use frame_support::{parameter_types};
	// use frame_system::EnsureSignedBy;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	};

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

	type Block = frame_system::mocking::MockBlock<Test>;
	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			EvmAcl: pallet_evm_acl::{Pallet, Storage, Config<T>},
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub BlockWeights: frame_system::limits::BlockWeights =
			frame_system::limits::BlockWeights::simple_max(1024);
	}
	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Call = Call;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
	}

	impl pallet_evm_acl::Config for Test{}

	fn new_test_ext() -> sp_io::TestExternalities {
		init_logger();
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		let evm_acl_genesis = pallet_evm_acl::GenesisConfig::<Test>::default();
		evm_acl_genesis.assimilate_storage(&mut t).unwrap();
		t.into()
	}

	#[test]
	fn simple_test(){
		// println!("AAAA");
		new_test_ext().execute_with(||{
			let v = EvmAcl::value();
			println!("{}", v);
		});
	}

	#[test]
	fn set_uri_basic_test(){
		new_test_ext().execute_with(||{
			let from = H160::from(hex!("0000000000000000000000000000000000000001"));
			let c_addr = H160::from(hex!("0000000000000000000000000000000000000002"));

			// "aufs://0x0000000000000000000000000000000000000001"
			let domain_str = format!("aufs://{:?}", from);
			let path_str = "/Web3Tube/movie1.mp4";
			// 0x0101010101010101010101010101010101010101010101010101010101010101
			let cid_bytes = [1u8;32];

			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::FixedBytes(cid_bytes.to_vec()),
			];
			let log_bytes = ethabi::encode(&tokens).to_vec();

			// "aufs://0x0000000000000000000000000000000000000001/Web3Tube/movie1.mp4";
			let check_key_1 = format!("{}{}", domain_str, path_str);

			// insert or modify
			let value = EvmAcl::uri(&check_key_1);
			log::info!("before set_uri, {}: {:?}", check_key_1, value);
			assert_eq!(value, None);

			EvmAcl::set_uri(from, c_addr, log_bytes).expect("set uri failed");

			let value = EvmAcl::uri(&check_key_1);
			log::info!("after set_uri: {}: {:?}", check_key_1, value);
			assert_eq!(value, Some(cid_bytes));

			// delete
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::FixedBytes(vec![0u8;32]),
			];
			
			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_uri(from, c_addr, log_bytes).expect("set uri failed");

			let value = EvmAcl::uri(&check_key_1);
			log::info!("after set_uri: {}: {:?}", check_key_1, value);
			assert_eq!(value, None);

			// delete path not exsit
			let path_not_exist = "/Web2Tube/movie1.mp4";
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_not_exist.to_owned()),
				Token::FixedBytes(vec![0u8;32]),
			];
			
			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_uri(from, c_addr, log_bytes).expect("set uri failed");

			let check_key = format!("{}{}", domain_str, path_not_exist);
			let value = EvmAcl::uri(&check_key);
			log::info!("after set_uri: {}: {:?}", check_key, value);
			assert_eq!(value, None);
		});
	}

	#[test]
	fn set_authorization_basic_test(){
		new_test_ext().execute_with(||{

			let delta_height = 200;
			// let current_block = System::block_number() + 200;
			System::set_block_number(System::block_number() + delta_height);

			let from = H160::from(hex!("0000000000000000000000000000000000000001"));
			let target_addr = H160::from(hex!("0000000000000000000000000000000000000002"));
			let c_addr = H160::from(hex!("1111111111111111111111111111111111111111"));

			// "aufs://0x0000000000000000000000000000000000000001"
			let domain_str = format!("aufs://{:?}", from);
			let path_str = "/Web3Tube/dir1";
			let set_height = 300u64;

			// read authorization 
			let rw_type = SET_READ;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(rw_type.into()),
				// Token::Uint(U256::from(rw_type)),
				Token::Address(target_addr),
				Token::Uint(set_height.into()),
			];

			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_authorization(from, c_addr, log_bytes).expect("set failed");

			// example: aufs://alice/Web3Tube@read#0x570da6…12f5dc : height
			let check_key = format!("{}{}@read#{:?}", domain_str, path_str, target_addr);
			let ret = EvmAcl::authorization(&check_key);
			assert_eq!(ret, Some(set_height));
			log::info!("{}: {:?}", check_key, ret);

			// write authorization
			let rw_type = SET_WRITE;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(rw_type.into()),
				Token::Address(target_addr),
				Token::Uint(set_height.into()),
			];

			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_authorization(from, c_addr, log_bytes).expect("set failed");
			let check_key = format!("{}{}@write#{:?}", domain_str, path_str, target_addr);
			let ret = EvmAcl::authorization(&check_key);
			assert_eq!(ret, Some(set_height));
			log::info!("{}: {:?}", check_key, ret);

			// delete authorization
			let rw_type = SET_WRITE;
			let set_height = delta_height - 50;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(rw_type.into()),
				Token::Address(target_addr),
				Token::Uint(set_height.into()),
			];

			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_authorization(from, c_addr, log_bytes).expect("set failed");
			let check_key = format!("{}{}@write#{:?}", domain_str, path_str, target_addr);
			let ret = EvmAcl::authorization(&check_key);
			assert_eq!(ret, None);
			log::info!("{}: {:?}", check_key, ret);
		});
	}

	#[test]
	fn set_delegate_basic_test(){
		new_test_ext().execute_with(||{
			let from = H160::from(hex!("0000000000000000000000000000000000000001"));
			let domain_str = format!("aufs://{:?}", from);
			let path_str = "/Web3Tube/dir1/movie.mp4";
			let target_addr = H160::from(hex!("1111111111111111111111111111111111111111"));
			let target_addr2 = H160::from(hex!("2222222222222222222222222222222222222222"));

			// append first delegate address
			let set_type = SET_READ;
			let is_remove = false;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_type.into()),
				Token::Bool(is_remove),
				Token::Address(target_addr),
			];
			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_delegate(from, log_bytes).expect("set failed");

			let set_type_str = {
				if set_type == SET_READ{ format!("read") }
				else if set_type == SET_WRITE{ format!("write") }
				else{ format!("undefined") }
			};

			// aufs://alice/dir1@_delegated#read
			let check_key = format!("{}{}@_delegated#{}", domain_str, path_str, set_type_str);
			let value = EvmAcl::delegate(&check_key);
			assert_eq!(value, Some(vec![target_addr.clone()]));
			log::info!("{}: {:?}", check_key, value);

			// append second delegate address
			let set_type = SET_READ;
			let is_remove = false;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_type.into()),
				Token::Bool(is_remove),
				Token::Address(target_addr2),
			];

			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_delegate(from, log_bytes).expect("set failed");

			// aufs://alice/dir1@_delegated#read
			let set_type_str = {
				if set_type == SET_READ{ format!("{}", STR_READ) }
				else if set_type == SET_WRITE{ format!("{}", STR_WRITE) }
				else{ format!("undefined") }
			};
			let check_key = format!("{}{}@_delegated#{}", domain_str, path_str, set_type_str);

			let value = EvmAcl::delegate(&check_key);
			assert_eq!(value, Some(vec![target_addr.clone(), target_addr2.clone()]));
			log::info!("{}: {:?}", check_key, value);

			// remove 0x1111...1111 address
			let set_type = SET_READ;
			let is_remove = true;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_type.into()),
				Token::Bool(is_remove),
				Token::Address(target_addr),
			];

			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_delegate(from, log_bytes).expect("set failed");

			// aufs://alice/dir1@_delegated#read
			let set_type_str = {
				if set_type == SET_READ{ format!("{}", STR_READ) }
				else if set_type == SET_WRITE{ format!("{}", STR_WRITE) }
				else{ format!("undefined") }
			};
			let check_key = format!("{}{}@_delegated#{}", domain_str, path_str, set_type_str);
			let value = EvmAcl::delegate(&check_key);
			assert_eq!(value, Some(vec![target_addr2.clone()]));
			log::info!("{}: {:?}", check_key, value);

			// delete 0x1111...1111 address
			let set_type = SET_READ;
			let is_remove = true;
			let tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_type.into()),
				Token::Bool(is_remove),
				Token::Address(target_addr2),
			];

			let log_bytes = ethabi::encode(&tokens).to_vec();
			EvmAcl::set_delegate(from, log_bytes).expect("set failed");

			// aufs://alice/dir1@_delegated#read
			let set_type_str = {
				if set_type == SET_READ{ format!("{}", STR_READ) }
				else if set_type == SET_WRITE{ format!("{}", STR_WRITE) }
				else{ format!("undefined") }
			};
			let check_key = format!("{}{}@_delegated#{}", domain_str, path_str, set_type_str);
			let value = EvmAcl::delegate(&check_key);
			assert_eq!(value, None);
			log::info!("{}: {:?}", check_key, value);

			// let check_key = format!("{}{}@_delegated#{}", domain_str, path_str, "write");
			// let value = EvmAcl::delegate(&check_key);
			// log::info!("{}: {:?}", check_key, value);
		});
	}

	#[test]
	fn combine_test1(){
		new_test_ext().execute_with(||{
			// A delegate to C
			// B author B through C
			let owner = H160::from(hex!("0000000000000000000000000000000000000001"));
			let c_addr = H160::from(hex!("cccccccccccccccccccccccccccccccccccccccc"));
			let user = H160::from(hex!("0000000000000000000000000000000000000002"));
			let other_user = H160::from(hex!("2000000000000000000000000000000000000000"));

			// "aufs://0x0000000000000000000000000000000000000001"
			let domain_str = format!("aufs://{:?}", owner);
			let path_str = "/Web3Tube/movie1.mp4";

			let set_value = SET_READ;
			let is_remove = false;
			let delegate_tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_value.into()),
				Token::Bool(is_remove),
				Token::Address(c_addr.clone()),
			];
			let log = ethabi::encode(&delegate_tokens);
			let from = owner.clone();
			EvmAcl::set_delegate(from, log).expect("Set delegate failed");

			// let check_key = format!("{}{}@{STR_DELEGATED}#{STR_READ}", domain_str, path_str);
			// let value = EvmAcl::delegate(&check_key);
			// log::info!("{:?}", value);

			let height = 100u64;
			let set_value = SET_READ;

			let authorize_tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_value.into()),
				Token::Address(user.clone()),
				Token::Uint(U256::from(height)),
			];
			let log = ethabi::encode(&authorize_tokens);
			EvmAcl::set_authorization(user, c_addr, log).expect("Set authorization failed");

			// aufs://alice/Web3Tube@read#0x570da6…12f5dc: height
			let check_key = format!("{domain_str}{path_str}@{}#{:?}", STR_READ, user);
			let value = EvmAcl::authorization(&check_key);
			log::info!("{}: {:?}", check_key, value);
			assert_eq!(value, Some(height));

			// set authorization to other user
			let height = 100u64;
			let set_value = SET_READ;

			let authorize_tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(set_value.into()),
				Token::Address(other_user.clone()),
				Token::Uint(U256::from(height)),
			];
			let log = ethabi::encode(&authorize_tokens);
			EvmAcl::set_authorization(user, c_addr, log).expect("Set authorization failed");

			let check_key = format!("{domain_str}{path_str}@{}#{:?}", STR_READ, other_user);
			let value = EvmAcl::authorization(&check_key);
			log::info!("{}: {:?}", check_key, value);
			assert_eq!(value, None);
		});
	}

	#[test]
	fn combine_test2(){
		new_test_ext().execute_with(||{
			let user_a = H160::from(hex!("0000000000000000000000000000000000000001"));
			let user_b = H160::from(hex!("0000000000000000000000000000000000000002"));
			let c_addr = H160::from(hex!("cccccccccccccccccccccccccccccccccccccccc"));

			let domain_str = format!("{}{:?}", AUFS_PREFIX, user_a);
			let path_str = "/Web3Tube/movie1.mp4";

			let delegate_set_value = SET_READ;
			// let delegate_set_value = SET_WRITE;
			let target_addr = &c_addr;
			let is_remove = false;
			let delegate_tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(delegate_set_value.into()),
				Token::Bool(is_remove),
				Token::Address(target_addr.clone()),
			];
			let log_bytes = ethabi::encode(&delegate_tokens);
			EvmAcl::set_delegate(user_a, log_bytes).expect("Set delegate failed");
			// aufs://alice/dir1@_delegated#read : [SC_1]
			let check_key = format!("{}{}@{STR_DELEGATED}#{STR_READ}", domain_str, path_str);
			// let check_key = format!("{}{}@{STR_DELEGATED}#{STR_WRITE}", domain_str, path_str);
			let value = EvmAcl::delegate(&check_key);
			assert_eq!(value, Some(vec![c_addr.clone()]));
			log::info!("{:?}", value);

			// append read authorization
			let authorization_set_value = SET_READ;
			let set_height = 100u64;
			let target_addr = &user_b;
			let authorization_tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(authorization_set_value.into()),
				Token::Address(target_addr.clone()),
				Token::Uint(set_height.into()),
			];
			let log_bytes = ethabi::encode(&authorization_tokens);
			EvmAcl::set_authorization(user_b, c_addr, log_bytes).expect("Set authorization failed");

			// aufs://alice/Web3Tube@read#0x570da6…12f5dc : height
			let check_key = format!("{}{}@{STR_READ}#{:?}", domain_str, path_str, target_addr.clone());
			let value = EvmAcl::authorization(&check_key);
			// assert_eq!(value, Some(set_height));
			log::info!("{:?}", value);

			// append write authorization
			let authorization_set_value = SET_WRITE;
			let set_height = 100u64;
			let target_addr = &user_b;
			let authorization_tokens = [
				Token::String(domain_str.to_owned()),
				Token::String(path_str.to_owned()),
				Token::Uint(authorization_set_value.into()),
				Token::Address(target_addr.clone()),
				Token::Uint(set_height.into()),
			];
			let log_bytes = ethabi::encode(&authorization_tokens);
			EvmAcl::set_authorization(user_b, c_addr, log_bytes).expect("Set authorization failed");

			// aufs://alice/Web3Tube@read#0x570da6…12f5dc : height
			let check_key = format!("{}{}@{STR_WRITE}#{:?}", domain_str, path_str, target_addr.clone());
			let value = EvmAcl::authorization(&check_key);
			log::info!("{:?}", value);
			if delegate_set_value == SET_WRITE{
				assert_eq!(value, Some(set_height));
			}
			else{
				assert_eq!(value, None);
			}
		});
	}

	#[test]
	fn ethabi_test(){
		init_logger();
		// let domain = Token::String("aufs://0x0000000000000000000000000000000000000001".to_owned());
		// // let domain_str = match &domain{
		// // 	Token::String(s)=> s.clone(),
		// // 	_ => {return}
		// // };
		// // let x = domain.as_ref();
		// let domain_str = domain.clone().into_string().unwrap();
		// println!("{}", domain_str);

		let from = H160::from(hex!("0000000000000000000000000000000000000001"));
		// let c_addr = H160::from(hex!("0000000000000000000000000000000000000002"));

		// "aufs://0x0000000000000000000000000000000000000001"
		let domain_str = format!("aufs://{:?}", from);
		let path_str = "/Web3Tube/movie1.mp4";
		// 0x0101010101010101010101010101010101010101010101010101010101010101
		let cid_bytes = [1u8;32];

		let tokens = [
			Token::String(domain_str.to_owned()),
			Token::String(path_str.to_owned()),
			Token::FixedBytes(cid_bytes.to_vec()),
		];
		let log = ethabi::encode(&tokens);

		// let mut fix_bytes = vec![0u8;32];
		// let token_params = [Token::String, Token::String, Token::FixedBytes(bytes32)];
		let token_params = [ParamType::String, ParamType::String, ParamType::FixedBytes(32)];
		if let Ok(token_vec) = ethabi::decode(&token_params, &log){
			for t in token_vec.iter(){
				log::info!("{:?}", t);
			}
		}
	}

	#[test]
	fn evnet_hash_exmple(){
		let params = vec![
			EventParam{ name: "domain".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "path".to_string(), kind: ParamType::String, indexed: false, },
			EventParam{ name: "cid_bytes".to_string(), kind: ParamType::FixedBytes(32), indexed: false, },
		];

		let event = ethabi::Event {
			name: "$SetURI".to_string(),
			inputs: params,
			anonymous: false,
		};
		let ev_hash = event.signature();
		assert_eq!(ev_hash, H256::from(hex!("89fe02195420686d75437d76eb54150bb43b2e19b6e17c8f6be110ba22f9a0f2")));
		println!("0x{:?}", ev_hash);
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use log::*;
	use std::{io::Write,};
	use env_logger::Builder;
	use chrono::Local;
	use sp_core::H256;
	use ethabi::{Event, EventParam, RawLog};
	use sha3::{Digest, Keccak256};

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
		let method_hash = H256::from_slice(Keccak256::digest(method_bytes).as_slice());
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
		let item_vec = path.split("/").collect::<Vec<&str>>();
		// item_vec.insert(0, "");
		let mut check_key = String::new();
		for (i, item) in item_vec.iter().enumerate(){
			// println!("[{}] + [{}]", check_key, item);
			if i!=0{
				check_key.push_str(&format!("/{}", item));
			}
			println!("[{}]", check_key);
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