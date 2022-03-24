use frontier_template_runtime::{
	AccountId, VoteElectionConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	// AccountId, AuraConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_vote_election::sr25519::AuthorityId as AuraId;
// use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
					// authority_keys_from_seed("Charlie"),
					// authority_keys_from_seed("Dave"),
					// authority_keys_from_seed("Eve"),
					// authority_keys_from_seed("Ferdie"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		vote_election: VoteElectionConfig{
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		// aura: AuraConfig {
		// 	authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		// },
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				// map.insert(
				// 	// H160 address of Alice dev account
				// 	// Derived from SS58 (42 prefix) address
				// 	// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				// 	// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
				// 	// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
				// 	H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
				// 		.expect("internal H160 is valid; qed"),
				// 	pallet_evm::GenesisAccount {
				// 		balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
				// 			.expect("internal U256 is valid; qed"),
				// 		code: Default::default(),
				// 		nonce: Default::default(),
				// 		storage: Default::default(),
				// 	},
				// );
				// map.insert(
				// 	// H160 address of CI test runner account
				// 	H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
				// 		.expect("internal H160 is valid; qed"),
				// 	pallet_evm::GenesisAccount {
				// 		balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
				// 			.expect("internal U256 is valid; qed"),
				// 		code: Default::default(),
				// 		nonce: Default::default(),
				// 		storage: Default::default(),
				// 	},
				// );
				// map.insert(
				// 	// H160 address of CI test runner account
				// 	H160::from_str("4718b814eF23Fa23318E49C2Ee395931F41AEBEB")
				// 		.expect("internal H160 is valid; qed"),
				// 	pallet_evm::GenesisAccount {
				// 		balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
				// 			.expect("internal U256 is valid; qed"),
				// 		code: Default::default(),
				// 		nonce: Default::default(),
				// 		storage: Default::default(),
				// 	},
				// );
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3637CCeE721E0b3D9e3712C4c5DcDbC20b232bCE")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0x52b7d2dcc80cd2e4000000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("B5C07593B54d9CB07bC48FbB6f580ccD065Ae942")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0x52b7d2dcc80cd2e4000000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0000000000000000000000000000000000000000")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0x000000000000000000000000000007d0")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);

				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a91f35c7cd81c2d18da5ba5f2bec8bcb9f012616")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d46c05ef221cecdb06d8465f72f06ba80536727c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("23050554ba27821edb0753bf8533971d4c08cca4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d850c6305f0586080640f78e837eaca1bbdb7497")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8ae560e00bda5f9c22ab8b73d1b4f2fb36d7d814")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("edacaad903c805044f66a99bbfd0b7996cbcd256")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9a2a972323891d00c829127f04abe3324a321670")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("32d13c3f8070806df4f4e8148489c3cc965cd2e9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cc1a10754d432b9b241213b0569e6186065117ae")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c85d7b96d01a7866c74f9e93e211d087b4c2cf73")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3d3c7ce9d026317c7d74bdedeb0b87b33945d591")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("48ce3068437d9ea0c4d8db5b022d4208749fec25")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c7fcd47e6ccb3a0096303028daef393aaa5b5de3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("06fea225520c2193f218b9b1bccdc67e1e9e7158")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("efa25cf24dc32f5135d6a628ef2c21fb4e806232")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("72a389463c20e50c7890dde00ace41d44bb736bf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ef66652f6fad7a3c9680175660656c5c11237c8f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("eddffb13b812a7e1a1f07796d4f9419c45c1bbae")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("331557338d78854db147d4c93e14044f3b437a78")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0529c4a772d971cd547ef5d8d6b69394bc6f24cf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dd69cf5525bfdff12cf242e57ddf207ceea4aba9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e98ef175efd5932a4c0c3130719711c9c2f0c895")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("318766ff33acbaebb23a07c535b7166985a9a5d8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4ea2ae95b4480210e5ac2a37525a10a561a89398")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d52ed6a0836273f29418e3a2b1a47bae278224dc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("edf4f56749d03228aa2bda91044888d78d05c78c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("29d848bd326ed3482b76deede33835c2f8284471")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("427744dc0df8da5c2eccc54b2e1f38a43d6522ab")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1adaec78af21a5ef16f47eeaa03d500a7f61b439")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("de4014a626552798a689bc3953abb7342168624e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("feb58c03be91288ed918ff50a5bbf6cc263a5f18")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f615554f8295b5f50c9d5ecff1d8ba195bf29323")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("714b825e26525ab3df7f841bde3fd37e1ea03932")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f179472dcc0e485cab620d0f031d0e818b58a456")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("98aa4300c4bd7ddd3474e0b5f12940a2a408abfa")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("78fb85daf6482b2bedfaa944f48c5905977ece70")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f1cba476da90cf82b4ebc0269e9890d0f4da5fd5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b74cbd122191f50f2332865795f1474b6c6f054f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b674525a7dd806817c866dcbb3ab6e439c43fc21")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ce63ff229b895ea26c76094c2bcf85aaf36df10d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9f1fad6e6b3eed3c67e4e8ac585024f9d8e9839e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("15d70f34965ba3a611ebb11cf422a4e484f87f62")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a8da412ace304d7151275d24fdbbd0d315a28660")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("09bcc92855ccec111504529a0f37a7eb39362a23")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fad56d80d9175f8b403e27e9187d4d2d52d39c6d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("901d86b0570c9749147be6d94e5ca512388389d0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f3c146de574598204ae91ba269a0b6cd799a3b00")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("10326b044c699c5b1830db2e5821d8cd7a8619d3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c79fa63d7c53f3958acb87d511aea4903fdbb522")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("317f754e96efa2c852f7afbcbe5478c8a0c390ea")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bb539368be66d769b856f44ac31119ac5d3a2876")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("267ef4ce3a25362c5b1f5beea1e898d51604a8d3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ba44ec1690769fae85b0750885621115e42173a7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5ee04db842046c2698786c7cff33ed4afc4adb5b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f96e0e21013898db97ae5160bad55b855adf2428")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d151b8122117ea4f647bb2723a09590738220294")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("12e6038693f2144990ae369371e3ce0d5ccc2767")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2a8f398e5780a3084659dbea7d8881a1638d083c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4603ebb9ddaf2edf2dd168340723bf1a3b0d8c56")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("239c69cba763dc04a3548f1a62746cbd159b096b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("04e5b2600635b0e1b70baceecebbfdc64a56eb80")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c40de73ac35bb4d4fcdcd61b0799182995b18a03")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("89d8e1e1e20011264b5fe6d5029e6220467be9e5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("401fb14768ab51f828ac63dfb150dd827fa183e6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("95d41985a11868033f478d4a2b6b5a8a51f83f6f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8460cd060c3d3ac62f4c777810c31a9e58fa4317")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7740777fa938185b8c55926ca7bde95c0b85d63c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("233e7fa2462f841da5f18f653ffbfaef772fee97")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cf5d0183171d180f7034ab48cb2728931ef4c299")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d6680888897a83761d166f8c4160fefb6af6c3b4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e77f34449fae06f26c2b34b4e1ad66f30a385914")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1751c4d0d0d52879444b8d0a4ea1bdd39efb2369")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f366affe9287a565787128daf9d19bfdd226fd41")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8df983a83fb249fb3ee58cbc74e98a6e3db5d873")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ea5d7832bd046dea1d660f9f1500c0443f780656")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5c5999d92eda8b2d108c9adfc224db31352a9858")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bdbd413d001563c7415422a0d544888cdb733d1e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6d34d16dbcb50a01ceb0276b66f3e28733e52568")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dfffe7eec2e5fcb346ebfd2c539cddfd599f96e7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c3dc54f7a0f0b5a842cedde80f5deea1a26a3c11")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("92c3f90baa80f7c5bff6fe39f14ef8b764ba0d8a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1bb09c66b189f99b61c4ec25d29fa9755bef3e7e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2d9bd4573a1b05d5c41d66f189dd3dc71adf8eae")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c60ffb399587a5d8511df75ef1473a25c9a218a0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("93744037fc82e537f0cd2e4745d2dee639dc00c5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("36c55358d90e91d353cc1eb37caeffab9a576563")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d1707863ee9ddf6970d9a035d674eb1e8186b209")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0d1d50e76c09dbf3fa492f185d83f4486dcf494c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("138f5a05565265fe446e6c94e3e19e543edc53e0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8d3f2563f443f1e863990609d6864dea3f7e5ca7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8cace83d8e0bef1d6901ae477b749778d3e40223")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("81c775c1591860c9911dc60ab853626aaf9d69da")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c3d75eeb29651f99d4a4aa7967968f0aca7579b6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("510ee58ceabb32a1283873e9bea1ced7ddee62bb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ccedc41d501391cde2e8ecf72a935b2879df268e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b31b46b6c2af9fe7d7d10e23f2231666944923f8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("36ca53daa1156e8d82b9cf4aa5100a9953dd4f98")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c6bb31c91e326c13b43b5a79c4b49fad345866d8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fcaf92bb006ebb5be81bafa9eb60132deab3dfdb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5dc621f8b0255dabc6798c66df0c731f848d6558")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5a0639f202d1470ab2cf39e651af623f780c169f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ea89ab5a212ad69c998e097f4d29243e408ddf73")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dea569ae08ded9e328545fb18b042e67fa757951")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("635d016ffdccfa9efd2cf6dd920b012944299117")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3ebbc58c818bb91836cf2419c2d2f4795b6a652c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a26b1b50fef63ab2d51700107135bfe684e3011d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("87a33596ddbe3fe9e4b889a12ac569e2fadf0ccf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5b9b94add327314feb261a615e1ad6fb1a98e839")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("511806859012a49493a0d9f196d14f0a91891b5f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cb743c296783131b09946e4de485a2c5e2351880")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("caec5f5960b206000b318e451deb109eb03ecb58")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e9b47e42505bbc93b83fcee0dca92a0a9df2dbd3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8dedca67ef1c3a7fe56383208c3ac8e8149bbb19")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e12ed9db2a3fde70f353544469c38d9ab4738ee3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f7d94d9952019815b9cb35d3949355a269131512")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("410e59907adc8c99557f697612907a4fef24e004")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2d788ee75f57337a002dd93f0d7e60e004fd28da")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1aec14d698a9449945fd97a42f9f213961f243a5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f94827233a62af5db9c5be637a9200344b9204f4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5e8c35cb2b177efaa89b800391e01d37bbb6171d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("22e6225fae910f04b529b2faf219981323583cf7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3c381f791294de3c233ffdfbbc4e538e220c8536")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d9baa5c817be44c30b46a7de0d485b1947f1ff93")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("db6c62a64a557bd67da51ce4777ddf665ec89cb5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c2a0ccf83bc8c217db49704e706e161cb90e8d1c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b3f7c204bdb2f242bf67ce33d4ee59667a84a06b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ad2580772a974cda7aeef1b114954e957de32978")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2273b07cb4f9260f4f43c99c2527dc689cc8503f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("02113281c73807902857cea6a347778d0d729087")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a1f35ae503642db28ab7d61371ebd578e41a6180")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2fee1e55ccebf433607b775443b71cfb138cc191")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ecefd35a30cc69752505563feceea62196261540")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0c40ac3b1f5f27a2f1ebe2c0468bbad6bd4a2839")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("98e442caf413a6589093741ce895585d777f7d84")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e70d0deb617dbee968d0d89408255343d795afd1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f5c61d620a838dd87d8a99b3c87e55a06d6d0486")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b9c4c90ff1e4b8b2da3c0119cb28314c55e1ecf1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e1daf3f2097e85d610da21de8a3e0394c01456d3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("75e58bf2a7aae229c106b25c63b8a2d1ad3ff374")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b5dbfca857767240bddcc3eac3ac286840111a11")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3f54be015a87c6bc9134fa099a42eb4721cbcd8b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("239a7e24d9f8586ed5cf89b38c18b625c73ba7dc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e9126a20e7d40368943d73c5127d4b5e75d32dfe")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("765488a309e08ad280b98cbe6c3b89d76d412c1c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("48794ab617a77e1ee3e33c8e7a0443660ced46c2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("95dacd6f04873f75d5b979a04187d23c609be9df")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dea4912176b7e6c70d5b42e6e8aa3ec2fe7de513")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("17da432f8ceb7dfff946dd9ed0235b6b5aa6ba7c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("790573fa6438e92c956fff0f0ce25f0c4b340e98")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f21236f52c6523d0b4f1330b3bf0a7073e0108de")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("60aca35bef468ecefecda0b28f4492a0fa51f721")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c3888123fe80669942657f8eaf6f5e5816638325")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e75c334a8cd8781de4cfb5156dcfe1b12eb86f7e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("438b4f96ea31f635b0386a9a978ee657a8f359e2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("01f3541bed30ed0a7775e6325a14ea99bd1f6b20")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ad39069115d13149415b37ecdd4c1c2921a419d2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1f816f7d8773bea4159eb17c93f061ad6253d3e6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("68bc7391b8c3400a635ff907a5fc27cffc299e39")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("71ffbd23bfd8ca2cab240e16005ef3e0420474a3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("50eb88e9ef07a0ddb6803a7dacdd7c6069ea36ba")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5ac5f4164865ce4fc549cb21808171d014d11441")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a7f4d84852bb0ce08546d9a45174c29c7446b1be")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("93002ae20fe78df48ac3f3e96d6b1bfea6eac3a3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("521ffd4f61d5c3256e4a854ace95c582b3e61c98")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ae9f4e9a5732d93d96e362d771b1c2792e7577b1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("289d9a02c4aa28a3d4d7bf31dbccb6e4ab303af4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b5bedc85b75447a2b35650dbadfcf16ab27ed4ba")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ea7e613ea31c637284eb6e7defee9f04ca8416de")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bce802216405532ea2d4f9bc7a39f634e3771083")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("82072ee8c5155dacc52e45307ad0a37cd2903ef4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cc54ffb6e1a73f50bcfef963042eae64fe852aab")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("69e4a86a238a51715430098a1bce4ba06e53e925")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8128998d805e7019e7ea5570248e207f761b13fc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e229f0942b5a18f6dcb7e324fdf5b102c000fdda")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("59dc185ad01000fe5554c26c04aebaef351e9e94")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("404bf17d4b211c8d7c8849467518666dcb3a60a9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("509788e1e40f3ae5207d4f09ab42adcd1ebd6e52")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("42604aea980d269c5a742e3701a7d25e8f97cbe3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("18125f441dc4bdcdeed3b2da39dd8f4093d73ee4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("80811ef1abc80a192335e909de43a30ea39c5586")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("70543a7c78bd4548b6222ce6eeeb45954790dec0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b51b560cf1d4fbab7a36277d25aeb81b54a309d1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d938829f51e4342e6c2b1faa8ee04b5b2ad68bfc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0a927603b05dfe386c3234478d8534e7ed290752")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("15f1cd699e22c8dbc5a5960a2ef27b9b27a1393b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2dd60702f01173e4c4824d973fcd70120e792e75")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("17bd6b0812b8754c9d5e05aec8d0670140de8bdd")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("103621419d657258f8bba1470d18ad68ec0a6838")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1cab8095d04305ec5a060b24374c6a52b11fe866")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8e30fe4a39cad3381840917fe116d03bb0634d9d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("48d9f07610f6721e5ea72ad34fb6a9fd0e0fd239")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("eb830ba1029f1cecdb97abf3946c0391fe5bc830")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3a7df3eacf68230a1d5f22d0d7a7c9bff978e25a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6556e7a92cfc0e5f49288621b9a348c1c0abc0c3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ee105312f755f2536a50d0c763476dd317ed3a0e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("61bc57f949d415b61a40ce2ce40d239ac2d7294e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2eaaac9d4b4b69fd88050792f8a7c15aa143359c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e6a64e4a4fd7fa119ae6d78ff5d147676ce58407")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4362d2eca95998df9e085c608f9f6e22790e1c7e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b8d877c3954a59734fd9b4636ae7d12ada412776")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("57b1e079f0afab0b5420f42b9a5ccef3e9cfa2d2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b1ab190ca6bb3b5e01f1ba8048a805b09cddc380")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c813ca83a54f7723b59b4ed373a3ba3b5b934e53")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9caf29125921a98f8adcf57fb49c82301ae961ce")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f628927aa4677aa7d3355dc2b23e9ac92da18d07")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6e15d69eaecd44c19a08061aec97132de438d0f0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("74788c7ac6027d7c52e9e7823b27b8dadd36539f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2cca936af0bfcfa6bb29cbe3173f94c4281f11a0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("906fd078a317936e82558f762afbc813f151828f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("92c45198eacce08ea1e3a4945967da8056bae42c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("833a170aba59990ac725ad8ca1e67c1e22a082bd")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2d9a0f6f28e8b76368630292fb5f7bb429c00dc0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bba1a300bf280b7eeb51da66a5235ed32f600a82")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9db9d531c78181dfa0a9739f8bcaaa79124ace51")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("330ec2df457af9441709644baa2965f485e8259c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("591b30013605da1ca114d8bfa5f8ae2bd999bdcf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3dc192c175fe7e9e99fd2c61c7d0097ba9f47124")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("879af30f03be4efbb3a2c7ee635ddbbed32ead70")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("30fdb877ffaebee83736ece6ec496d75a9acb814")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fde73fb42713b38ea1472f40aacabff2be70800c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dec5284a15875b843ca64f5a8fd1cae76cc77ac8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ac434688d2d1ee1323b20e4dc101f7aa07aa8624")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("db487e84074f4d428d6a99e5cba86e3d71324ae6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bdb42621bde2241b8f99db93dd6eea8e330ce799")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6813ec8ea41fe156d99e6041ac9bd6ef84ae812d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("14aba83629b0f0c73dfca21f88e157586d1ade7f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("571ec742431ed0b1d57ced27fa0958dc7b1bac59")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("49844582c3c686031f695407c36dcdeec2f3d6b6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("81d2d2ba531aa7b569a8ac68abf5b641d8e70533")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e09b1e373262bd3b963b39e5e41cb7060c4c8a0f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("81edda37cd57a3a095bbe175f9b72bdcab83b321")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5d2d1322078c42aa8985c0291ec090ba1a24d29f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("869cbb1e4ff574ee37431b2cf44b0d1a8f01312b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8a56773e43b90f41054d6c5c99e501a26b831d69")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0cb93d1cdb83acebbd625bfef559649874452281")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6d8cb3b7dcdc4687fca30753f63d59b789e87930")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0fff855cad920adfb1eb63f51b0fa98322dfaed4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("67ae92effdf249872f5ce9c6a3316969ae0a05aa")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b94e4640714d20abb7fde037db121a992b3c731a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bb2db81a5b3762a26a336badf1a0e4fd4eae5978")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("da66da3817475d00a2cf71f372785970aaa36dfe")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e3b57ddfb2b474905b2532a3fb94c71b6594f813")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a47e13b8e59da350320c3a18ae9940587937b049")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8526d6b2ad7c250ebd3c89b9f1c00b9c147f50b7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("005417e707732ce82622ce82f20c5e4d2ac22c1f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("982a968183dd66fdee904ba83118e6d1fe9cf284")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9552f8b31458205655b2f01379ddfdf13c38fb37")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("669e214fad960c592a0d66aaba9bbeb65eb0e3ec")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("176dc6f5c9467607ddf9b81164ee70c927971800")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8f20ae4219fd72ad7a97ad71c406c08b32b787aa")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1e4d853747ccb89d317e7dfc44c9a896669f9c1e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6bc3390bf664e1ab44b3d3f562633eb7fb08d567")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1677fadf4e1210ba1964f8da32d06b8887007dbb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("02b33e09be9dd26b93499f1e3b5701a213da042e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1ba989ab4e6f1d770d3c36d5d9bf32f56d352d35")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("943e6853ec60fdd4f8608feef2ae9a251cd46fd6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6205771df8ff3eef129a6c80a0da52da6733753a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5e1fc4dda700d667b0bec9bf805f880e7fece7c8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("66e9a871d58eeb367363bc206eeec3ce55b87ce9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c201d9ba558169f0cc115e25e7922f77b80ead14")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("56a6aedaafd5a057c130ef4f9487f78d7a88283a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ec830e64efb1a1cc376c8fde17dd003ad49fd009")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b64bc2403e676d2c1bf36d5998da39bee0a7d3f6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("06338e8980c76af17bc116856b7b3df1c6cfb940")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b4409ae5ef2e0936029c45cf02d1ab4f3e174491")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3141a0536d9e50bb6f6576f0661eae013b61e205")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5b963965afab34a41cd07a4cf6b75dec0f9a7a91")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("38a8574e9b2640c08fc5a8ebf32dae24235a6ffb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b11b25a09b052c33ad7b96f04cfab25decea462b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("74e7ff812a4757b7daae770068a69cf84184b42b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7ae1f23845f3c4f34c2196f321c6c4509fd92a9d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5da445ad2c56d3ff5a7fdcb5394f983dd99e9182")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("39e0f5e80ff5e29fd47617e6ba9fc9397a254855")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("44861d48bb78d8eaf2c9f4eeda3e175116a9b9ca")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bfe7be3e420c6bde5013639d72cf11aa1e50ae0e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("61a125ab75dc04b1a605c79d4e44672625ade3a5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("283901a29c752a270ef7eefd9e142af5a6d37b8d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8c80a814ba3629aaa1929012f41008468f071046")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f37b10217b3a2039d34388ed5be8295c85ba277c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2d7377d04341e6638fbb93379b41816da53ec7a9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4620c77f6ecf41f3d77212e9adc2992f852ab71d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f0622f69277a245680fb73b491e22b036d03ba70")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ed737ad7a736811441f4079c627a114433c5dc5f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6c0e1cf01e24a4215b79d912a38623cb6a9a50bf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ca2532cfd2fc6118a1798ff813e22d3640e54d91")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("afcd4afff7585e00b8ea1f19ddc087412556d8a4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fac27bfda501ebe47182bea47bcaeea9643b87af")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d2000db364988951ff2049af79fa733e1476993a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f0863d0439a0ba180d6b318a9f83302728a60c04")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c573a8887e2373698eb6fa059245cce93584a223")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b8896f6b988a61fd9a15e950b4b84f8701a09ba2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cac1a78434f119141429e7e0971a88547925bbf6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fef3c4af30d9c12da9912e0bf1c815fdfed0df89")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("82b5f109dfa3d1f16ed536fba101f0d293f3cb58")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("79afea8387f4ee04881b883206b35b067a48f38a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("230842f7d55b28d49fbe33759edcf7ab17908cc7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6acc12ca175e9db0d053efd2f9953f31b422a564")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("942933f96ce7f13d2b176a042dcc78bba2128920")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0638722d73b29791d1dc2ef0c4b2b8f762e4f31c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d237dfaa68bcb68f15b2b0911908eb0d573c1959")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("daf52bce3fcb4df7cb9877b4df7ba3289a6df15c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("94d6968191155ddaeeea9b537542f73013567534")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("20b0b1a60ac721af48093ee61c55b76045e5d868")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("15eaae74da14a839507c956e64448e7dd33625a3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1570f075fc034edd881034711d5aa11c90b640bb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b0c0c95c3d827c6f7f3b7a3c5b7532e3b9ccc8c7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f8bf379a1123cf4d2cad73c267f2950cc4e31063")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("712e397480aa8484ff1478eac4a05e285225b11c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e58170b47b14557b8741f39763297d3c6a3a75af")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7f8a61971d52996141859c48a7398ede4c8af1d2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8b23f9dbf27d597d61752cbf40f9a3709c51fef7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b638f367b0e35f8865841e30db62ae5ac7c46d0a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6767090661227ff210faf8b8389795c1c298da77")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d2ea06af22f410e8e683b218e363aee78679065d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d4b7afd713c9991975333bb32800ae1a3ac069b5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c3b30b349e48d1e4007e61e743cd931c2c5efcc3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2ae632c1580318f83db4379b4edb7db4561757c3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("81e06abc7976b54ade0769ca846a64d34336a8af")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b7c3dea57e9ff7f74e571504046c86ff07d3d2e1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("05d5053e31a49ebc46dc4575386fdb60ce342b1f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7d700df6a46ec0af919beb3c330afbb3cece8bc2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("243fdd812eef1bafebd51b9132b9310678be610e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b6a9acc6d1e4d107a32a278f69ef560203b46bcf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5cb1b568c33b24cb05aefc58a3a713dd61ae5f72")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e5ef99ad845ac32fe11420de59c00cb0ea5a796d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2056391dcaf4e1def3b74402ffb0e90d45fdfa59")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("825f122a5be81c20290856684225cad5e91d02a6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f4fd58d2dd26c675aac6da81df9cdc77c1e45bea")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0b33ae5ab8cab6663b83c9e1e84c097e1151269c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("66ad2e71cfb0b16a2cc7cd09772fa7c522bc000f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8f9b2be34af5a1f809223670a69fc8c27f605aca")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7239d13074e908755162968ea8ae4e59c810c7c4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("974c8ffde7058bd215b319a75ab66d260fb4ae0a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bffae3d99f6507e51d0a2244b5c480a13e92b79b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f9712288e1f1251507af5aa2e7d88e80a411c9b1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cb6f5d52c8f9b19c9eeea862aa105e599edd3ff9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d4e4c7e07ff21dc05683e2ac5c592a595bbbb4d8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6aff67a1e27daea632562298fa12dbc60f2c6faa")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cb083907132d469a19b263fbb1ac3566d7b697d6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d1840209d01613c746e5adc97e7a860be49cf452")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a63aba09ac20a4402098f5cdee864117f63387bf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ad464ef845e86a98cd4d2fcb147384e62d813af5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4e15cc234632577203046cc775345bd0d7104cfb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f1a5fe35ef6d15b274a20496cbf375823b3e96da")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d5c1df095b28d156af7912e2df8c58b6f2ebdd3a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("55534ee3c5558eff7874199d4539e187472eb5eb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4a73d588dbd5119de936317214157f42f0f10ece")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6898a4b3d0d52a9eedc3bf523f0fc9c9d692906f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b45c15dfd0ccf2abecebb2f56c1073437870ccf8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9f13435ceeaef9ae74b89ad8048d7f6ae45abc88")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d077dbd553f374d4d3c996f3f778a601e96bd542")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a39cf4264bc660927aee854399a7b91f90e189f7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3e8816c26b2a0c9fea471c1ae2527703c28e01a4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("052c45796ba3a7a262da8c37e2c9ada5e9ed9c78")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7b0c2ce5ce2c1910a1a5d3d8f4aeed6e3120d423")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ec072a4f13af5a5a677b5745a7559e9db7b61cd7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fd526f739296d17ec0a28c35198b6649138af216")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ba3d18261602f666fd7352d90bb84616401aa0c9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9b43621a6344ced21abce4c740d13a24b7263e3a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8d060348385727ea7647185519564f0f0663bcb1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6297ae0fe264a344ae66b11ee323ab1b8ea5ec18")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0fca79f6a0122da183ed77b4b4c7f2e9bbb9f36a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("aff65d3a168078d6ea8d82fffe93e8d026004761")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f1dc1c906501dd6226664ca83ed0ed7c03ed4638")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("afddf7e3e6e664e81ff07df7e58b4e8bf8722e8e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6889f7f870baab22379e93c403de88b5af62ec4a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("febf7381cdb362807c10ff0a17b0f9951406fcff")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1d5e3796c56ef208eb6b45a1c2f9831cf726087a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a0e82e32fc23a869beddf33fc82e19e5f6043d61")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9b2315dfc5b7ca625bed3250cee2a6cee145e474")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d464e71e92bc01b2d2628ae145f9f95765f196ef")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("003b0535135576dc927f7234537947dfa09c89e0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e7169ec0642e50bd399c4c705688fd254f4df777")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("615866452d096f4d20e3d00b1a31d442c58eb1ae")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5a17d36bce3b58193fa09e2bf10542a3c92acf22")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("48c41fa8e796d83a40283bdb6eb40d5731794a1b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("65c86b321bc5dfd24aad090a229dfe76ab0d5676")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2b657af0ad045e7f8f8bc106fa79f9cd08555c63")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("da637e5a7cce71ca30e4d08adce9b26fa0128735")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("11fda4af43a22d1ac15fbabfd86c718d59bb5c0d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4a8b9aab8b71dee48c65a028be2108fda8991c76")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("449bd716d95ed33c29ffbecc8bed6714e3faeece")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cc2677736853a04799247a34e1efac9f5de6c6a9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("722d778369e5dd61ee0051d03d99d8ae066f37a0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dba7934636225c1605bb24c7f089abd12a750f26")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0de8039837e9f33aaca2ed249e11275940ab60bc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e4fbf5ff6baa66bceda19477d4c273a5e61102ac")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e675c732aeb1451c377218fe7043517cafd7fe6c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2ab19bca333dd2ae80a211bb93552842b998d6b3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("663e873fd6a02de15e3e6598f725a2bf4627ca75")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("341ebab21995fb46ed29548e60568ef985a4bec5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e2e23b39082c7ccd3f67f7c68255d1e15e4d2573")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b132bd2c1da94ff2bbd3d2a86be6880d2fed054d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bea083ae2b5486afa7c92b8beee259cbc86357fb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2cf773f9d85529f806fb4c243bd3930379a9cfb4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ce78157706f3f822edc8243f9bde0dea886a41cc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8301a960d55d9109a1991ae353e7085fc8d391b2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0ed6d90aeedb32a8fafb992f0a8308d44e499a56")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d97a8bb29820287b9bb9ccaf6925b151363a03cc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f1d84f26bab2bf82c2385c703996c882e269901a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1cfbfabc8ec8cea644ae0ecc0b7dae46c358863a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("669b02d6ca5e4f94b9eeb12abd29974489eeb4e4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("286a40f0d7faa9e2ca734f4befb8c3348f8f2eeb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c8a6f49721394509443fc2eccaf0d96c6f512f72")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("5585f5eee99fc2a700b18544a495a7583790deb4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3e091819f17f9ddf7a25ef8ed3d30fcd0827fbaf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8390c19ddb3200105c45c39e9e5569baa4baa8a0")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1561cd883c83ccca40d3b68c6ef06713951fd1a1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("592fb528616e43d340cff83564b54df9206767cd")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("505460ee571d31ae7288578f964db12dbcfc7aa9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c6ca5a3c42a4ac3879e27633aca6b2faba70a5d1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4ba175576d153d61ad97abf8258424b50fcd4f44")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0915ba368c2f941caa7a954719d0edae3a45d517")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7a10d1b5f722d8906efb7bfcded7b6842f409804")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("556e95d5cd9af4d7882ccec1e0c6a43bc9491814")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7a0fe210c64aa1a4b0e6d721350827b9709cc693")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ab3f2e50d1017bc4ce4f8252f1108e124e772a90")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ea9121da63ae78caf7cce50f383e373f7aa34e47")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8992c9beb30afaf267975419325855b884347ef8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f5b47b4df599f94e67e1cc6592317979cba44d3a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("eeeaefd75e9269ecf1937e52ace59f7f3d4b1b1a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4ee07255209049d9959635a6c8f95616ad4fb8e8")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a49fa59715f5fc182185732376a821aa8a352657")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("956c7111de078eb18b8fc2b828854bfe067577d2")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("da68d8e2e17415d7e2db1665818584f8989921b1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("321cd0e2cb7ba5537f8317172a55d410199e7330")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cb4d55308b4978d54c2f758aab43b9b617c42b43")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8a6b80f7bf3f5fdce4154cafa46625f6d6dfdc9a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("980dc208cd3c84dcc97bd6cd131037265094b189")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cd6d6d134f05016e8ec2dc6bd059a703d9d7877f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("17fdcd5cb780c8d88875c3797bdf4753f5ee080a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dba1912378182ba0d4ea8e927cdb1a30ecf7ca4b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("692c4db9984866482bd71fbb9bbacdc69b805ff3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("14bf2f033b8b720c004242695c47fe0fe545b0c9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cb7c71897bd897eeabed1321ab96c912a272616a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f7002bf4bb917c56fc4b7db9794aaa401f2feaff")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("76a36bf1c7e2476635f16afd7d254788db65acf3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f53785834ceefde60ca4b3b13d183c345c15b99a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a208dd8bc19789277a76d116f5caf11ba7c4d4b1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d8305f537a4f23eea9688b8ef4475ed03d4f36a4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("66fc170f48b462ce3a8de4ad6ba24b5b49a58605")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a2355f331b68a8cebee8602c2643ad492ebf92a1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1446bc73f4c01e80d6358e615817dc7de8f95a56")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4b310a9ae1255e9849f44e5db5f32da6cf2d621e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f40533b40f88ba8f759a74284d6de874f3aa6292")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("fdc652d997ed4c583eb0a3b5e0e63c161e557d9f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("366712eef5d9bfc670bbd804335b28da8d3bc9b7")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e42884ce554ea3a476982649f264d69cf12adee3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("851dc684f895ac7ae7ca76278f33941055a34393")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f821f9a2dfca67f58d1485e38a87ac685e35301f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b6ece30e9a37a771f42079e3904301c176dc5cb5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6edda19e774c2edaa18984b88ea60765ca3cc88f")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2f5dadb26f7c27c0f2dc351d28d645c25aa283ee")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3a3c7011a5803fa8f8e25f095aea232848689865")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b1d837ff2feaacc886436c4257e568813f150aae")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2002dc292657fb213b3593043640e2c4ddfb23c4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("32655d51684e17e83f7db63ded67a21d1ed938bc")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f2587e95f9c3714a225437f1280c3b956ebfd34e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("75f87efeb736711c8126e23a4e4927184e2c53b4")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("adf635bd9bba2307d0995e4fd3acb655e8f1996e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("3fce5909220c79fe2fc7d098a524e862b043af06")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f0166256eaaae99018ecd0770d4123977ee3f612")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c73716614f805c065b57b25563fd0699520964be")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("04a65e94abf93e294e1e4904c4a37f6fa9f8e326")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0cf9ac2a0a6477d0522c622c31ce7e1732a77505")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("07b5b51f2f250dabe45a29c5caacc284a9153a4d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("67247de5a56087f133776a890a2aad5f2e5f223c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e13d7724f9b919404e0be19e0f1adec45b1dcdb1")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f83c762741811dd75954b7b2d1eed9a1d62910df")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("4f7b46ef433f5eb44b69c1186afb2c2104fc4991")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("c9e3d70fb147ea5c8226c4e56771406f557d5a3c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e5960b52f2934a5bfc44709c45965eb12d01de8d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("dbe650bd7514b6c041f216463d24900f60648eea")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("812281fe8060061ff6611337c7cc4c8881fe14ea")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1ef6585ff38416118b868d9a68594d0553dfdd42")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("eaba71d3a50943af21962f6b557ffd1c444599ad")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("0d820dd917821985b313671272b1df863af86d52")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("d85e80fe0aa43781f238429f4996c83c5e2117d3")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("16cce4ec1caf1275ceb5114a84b0c08f8c0bb455")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("9146942747a2a13dc662f950cd4aba446b08a5fa")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("675e751e96d70075386363de922e58adcb638c86")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("e111a09e2c1f0b034df309820ef1d08e4f52b173")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("ec16c40f4872055feccff7a1d31e8b73fa70ea7d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b3e7c1e495127a037287d086ff9a6e41be8c74bb")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("45aa33b2d7e29be31794934ba8e5dc8dd47a7e26")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1d4a4755700f937074502c976b213c8309a45c0d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("26e27bf008dd86b0949783531e0c33e6442f5d2c")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a79eb12976073bcd1101fe98036fb2056224343e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6ba10227e9c464029d3460de871e5e7f95209b28")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("a0403ffa1185976a1081106df37aec4f7d78db9e")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("67c89a653183e1a917e8fb6bea39ac85ed41cf1d")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("49e9d45c0e13980356e2093be8cac3dc512ba68a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("69140347acf1283ca6827a0ca2d0968b0d3b4f94")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("f3f374b8c42d5e6a3a6caccd20a984dcf79d4fb6")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("bd984e24586fa42d2dbc994888564d41f5f8649a")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("8b65a3f537af63ff5979bf61102c3f62ae02f1f9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("2ccb7a94daa7ba8682c2ab7d2e0a5be2fef4b202")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("cea5844869de60cd6090d178b214dbc1b9bd75a5")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("b82ff90866bfda3f41c5af77a6da381493ebeacf")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("7ab384ca7f9a557dcae681ccaa8177fd4863bc42")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount { 
						balance: U256::from_str("0x56bc75e2d63100000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);

				map
			},
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}
