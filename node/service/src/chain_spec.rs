// Copyright 2018 Stafi Protocol, Inc.
// This file is part of Stafi.

// Stafi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Stafi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Stafi.  If not, see <http://www.gnu.org/licenses/>

use primitives::{Pair, Public};
pub use stafi_primitives::{AccountId, Balance};
use stafi_runtime::{
	BabeConfig,	BalancesConfig, ContractsConfig, CouncilConfig, DemocracyConfig,
	ElectionsConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, Perbill,
	SessionConfig,	SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, WASM_BINARY,
};
use stafi_runtime::constants::{time::*, currency::*};
pub use stafi_runtime::GenesisConfig;
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use babe_primitives::{AuthorityId as BabeId};
use im_online::AuthorityId as ImOnlineId;
use crate::fixtures::*;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "sfi";

/// Specialized `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn stafi_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../../testnets/v0.1.0/stafi.json")[..])
}

pub fn stafi_testnet_config_gensis() -> GenesisConfig {
    let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)> = get_vals();
    let root_key = get_root_key();
    // Add controller accounts to endowed accounts
    let endowed_accounts = initial_authorities.clone()
        .into_iter()
        .map(|elt| elt.1)
        .chain(get_more_endowed())
        .collect();

	testnet_genesis(
        initial_authorities,
        root_key,
        Some(endowed_accounts)
    )
}

/// Stafi testnet generator
pub fn stafi_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Stafi Testnet",
        "stafi_testnet",
        stafi_testnet_config_gensis,
        vec![],
        Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None)
}

fn session_keys(grandpa: GrandpaId, babe: BabeId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}


/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId) {
	(
		get_from_seed::<AccountId>(&format!("{}//stash", seed)),
		get_from_seed::<AccountId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_from_seed::<AccountId>("Alice"),
			get_from_seed::<AccountId>("Bob"),
			get_from_seed::<AccountId>("Charlie"),
			get_from_seed::<AccountId>("Dave"),
			get_from_seed::<AccountId>("Eve"),
			get_from_seed::<AccountId>("Ferdie"),
			get_from_seed::<AccountId>("Alice//stash"),
			get_from_seed::<AccountId>("Bob//stash"),
			get_from_seed::<AccountId>("Charlie//stash"),
			get_from_seed::<AccountId>("Dave//stash"),
			get_from_seed::<AccountId>("Eve//stash"),
			get_from_seed::<AccountId>("Ferdie//stash"),
		]
	});

    const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
    const STASH: Balance = 100 * DOLLARS;

    GenesisConfig {
        system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			offline_slash: Perbill::from_parts(1_000_000),
			validator_count: 7,
			offline_slash_grace: 4,
			minimum_validator_count: 4,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: vec![],
			presentation_duration: 1 * DAYS,
			term_duration: 28 * DAYS,
			desired_seats: 0,
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		babe: Some(BabeConfig {
			authorities: initial_authorities.iter().map(|x| (x.3.clone(), 1)).collect(),
		}),
		im_online: Some(ImOnlineConfig{
			gossip_at: 0,
			keys: initial_authorities.iter().map(|x| x.4.clone()).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
        ],
        get_from_seed::<AccountId>("Alice"),
        None
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        development_config_genesis,
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None)
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
        ],
        get_from_seed::<AccountId>("Alice"),
        None
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        local_testnet_genesis,
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None)
}
