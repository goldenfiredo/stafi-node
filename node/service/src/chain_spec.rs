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

use primitives::{ed25519, sr25519, Pair};
use stafi_primitives::{AccountId, AuraId, Balance};
use stafi_runtime::{
    GrandpaConfig, BalancesConfig, ContractsConfig, ElectionsConfig, DemocracyConfig, CouncilConfig,
    AuraConfig, IndicesConfig, SessionConfig, StakingConfig, SudoConfig, TechnicalCommitteeConfig,
    SystemConfig, WASM_BINARY, Perbill, SessionKeys, StakerStatus, DAYS, DOLLARS, MILLICENTS,
};
pub use stafi_runtime::GenesisConfig;
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;
use grandpa::AuthorityId as GrandpaId;
use crate::fixtures::*;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "sfi";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn stafi_config() -> ChainSpec {
    match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.1.0/stafi.json")) {
        Ok(spec) => spec,
        Err(e) => panic!(e),
    }
}

pub fn stafi_testnet_config_gensis() -> GenesisConfig {
    let initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)> = get_vals();
    let root_key = get_root_key();
    // Add controller accounts to endowed accounts
    let endowed_accounts = initial_authorities.clone()
        .into_iter()
        .map(|elt| elt.1)
        .chain(get_more_endowed())
        .collect();

    testnet_genesis(
        initial_authorities, // authorities
        root_key,
        Some(endowed_accounts),
    )
}

/// Stafi testnet generator
pub fn stafi_testnet_config() -> Result<ChainSpec, String> {

    Ok(ChainSpec::from_genesis(
        "Stafi Testnet",
        "stafi_testnet",
        stafi_testnet_config_gensis,
        vec![],
        Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None
    ))
}

fn session_keys(key: ed25519::Public) -> SessionKeys {
    SessionKeys { ed25519: key }
}

/// Helper function to generate AccountId from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate AuraId from seed
pub fn get_aura_id_from_seed(seed: &str) -> AuraId {
    ed25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate GrandpaId from seed
pub fn get_grandpa_id_from_seed(seed: &str) -> GrandpaId {
    ed25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, GrandpaId) {
    (
        get_account_id_from_seed(&format!("{}//stash", seed)),
        get_account_id_from_seed(seed),
        get_aura_id_from_seed(seed),
        get_grandpa_id_from_seed(seed)
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed("Alice"),
            get_account_id_from_seed("Bob"),
            get_account_id_from_seed("Charlie"),
            get_account_id_from_seed("Dave"),
            get_account_id_from_seed("Eve"),
            get_account_id_from_seed("Ferdie"),
            get_account_id_from_seed("Alice//stash"),
            get_account_id_from_seed("Bob//stash"),
            get_account_id_from_seed("Charlie//stash"),
            get_account_id_from_seed("Dave//stash"),
            get_account_id_from_seed("Eve//stash"),
            get_account_id_from_seed("Ferdie//stash"),
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
            keys: initial_authorities.iter().map(|x| (x.0.clone(), session_keys(x.2.clone()))).collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            current_era: 0,
            offline_slash: Perbill::from_parts(1_000_000),
            session_reward: Perbill::from_parts(2_065),
            current_session_reward: 0,
            validator_count: 7,
            offline_slash_grace: 4,
            minimum_validator_count: 4,
            stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
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
        aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
        }),
        grandpa: Some(GrandpaConfig {
            authorities: initial_authorities.iter().map(|x| (x.3.clone(), 1)).collect(),
        }),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
        ],
        get_account_id_from_seed("Alice"),
        None,
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
        get_account_id_from_seed("Alice"),
        None,
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
