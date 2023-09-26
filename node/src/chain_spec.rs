use serde::{Deserialize, Serialize};

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, GenericChainSpec, Properties};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use polkadot_primitives::LOWEST_PUBLIC_ID;

use cumulus_primitives_core::ParaId;

use myriad_runtime::{
	currency::{EXISTENTIAL_DEPOSIT, UNITS as MYRIA},
	AccountId, AuraId, Balance, BalancesConfig, CollatorSelectionConfig, CouncilConfig,
	DemocracyConfig, RuntimeGenesisConfig, ParachainInfoConfig, PolkadotXcmConfig, SessionConfig,
	SessionKeys, Signature, SudoConfig, SystemConfig, TechnicalCommitteeConfig, WASM_BINARY,
};

const DEFAULT_PARA_ID: ParaId = LOWEST_PUBLIC_ID;
/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = GenericChainSpec<RuntimeGenesisConfig, Extensions>;
type AccountPublic = <Signature as Verify>::Signer;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> (AccountId, AuraId) {
	(get_account_id_from_seed::<sr25519::Public>(seed), get_from_seed::<AuraId>(seed))
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn get_session_keys(keys: AuraId) -> SessionKeys {
	SessionKeys { aura: keys }
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

pub fn rococo_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/rococo-parachain.json")[..])
}

pub fn local_config() -> Result<ChainSpec, String> {
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Local",
		// ID
		"myriad_local",
		ChainType::Local,
		move || {
			genesis(
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial collators.
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					get_collator_keys_from_seed("Alice"),
					// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					get_collator_keys_from_seed("Bob"),
				],
				// Pre-funded accounts
				vec![
					(
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						124_999_990 * MYRIA,
					),
					(
						// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						// Balance amount
						10 * MYRIA,
					),
				],
				// Parachain id
				DEFAULT_PARA_ID,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: DEFAULT_PARA_ID.into(),
		},
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Development",
		// ID
		"myriad_development",
		ChainType::Development,
		move || {
			genesis(
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial collators.
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					get_collator_keys_from_seed("Alice"),
					// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					get_collator_keys_from_seed("Bob"),
				],
				// Pre-funded accounts
				vec![
					(
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						124_999_990 * MYRIA,
					),
					(
						// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						// Balance amount
						10 * MYRIA,
					),
				],
				// Parachain id
				DEFAULT_PARA_ID,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-development"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: DEFAULT_PARA_ID.into(),
		},
	))
}

fn genesis(
	root_key: AccountId,
	initial_collators: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	id: ParaId,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY.expect("WASM binary was not build, please build it!").to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|(account, balance)| (account, balance))
				.collect(),
		},
		collator_selection: CollatorSelectionConfig {
			invulnerables: initial_collators.iter().cloned().map(|(account, _)| account).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 18,
			..Default::default()
		},
		session: SessionConfig {
			keys: initial_collators
				.into_iter()
				.map(|(account, aura)| (account.clone(), account, get_session_keys(aura)))
				.collect(),
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		transaction_payment: Default::default(),
		parachain_system: Default::default(),
		parachain_info: ParachainInfoConfig { parachain_id: id, ..Default::default() },
		polkadot_xcm: PolkadotXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION), ..Default::default() },
		assets: Default::default(),
		democracy: DemocracyConfig::default(),
		council: CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
	}
}
