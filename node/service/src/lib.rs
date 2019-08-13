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

#![warn(unused_extern_crates)]

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;
use std::time::Duration;

use babe::{import_queue, start_babe, BabeImportQueue, Config};
use client::{self, LongestChain};
use grandpa::{self, FinalityProofProvider as GrandpaFinalityProofProvider};
use stafi_executor;
use futures::prelude::*;
use stafi_primitives::Block;
use stafi_runtime::{GenesisConfig, RuntimeApi};
use substrate_service::{
	FactoryFullConfiguration, LightComponents, FullComponents, FullBackend,
	FullClient, LightClient, LightBackend, FullExecutor, LightExecutor,
	error::{Error as ServiceError},
};
use transaction_pool::{self, txpool::{Pool as TransactionPool}};
use inherents::InherentDataProviders;
use network::construct_simple_protocol;
use substrate_service::construct_service_factory;
use substrate_service::TelemetryOnConnect;

pub mod chain_spec;
pub mod fixtures;

construct_simple_protocol! {
	/// Demo protocol attachment for substrate.
	pub struct NodeProtocol where Block = Block { }
}

type BabeBlockImportForService<F> = babe::BabeBlockImport<
	FullBackend<F>,
	FullExecutor<F>,
	<F as substrate_service::ServiceFactory>::Block,
	grandpa::BlockImportForService<F>,
	<F as substrate_service::ServiceFactory>::RuntimeApi,
	client::Client<
		FullBackend<F>,
		FullExecutor<F>,
		<F as substrate_service::ServiceFactory>::Block,
		<F as substrate_service::ServiceFactory>::RuntimeApi
	>,
>;

/// Node specific configuration
pub struct NodeConfig<F: substrate_service::ServiceFactory> {
	/// GRANDPA and BABE connection to import block.
	// FIXME #1134 rather than putting this on the config, let's have an actual intermediate setup state
	pub import_setup: Option<(
		BabeBlockImportForService<F>,
		grandpa::LinkHalfForService<F>,
		babe::BabeLink,
	)>,
	/// Tasks that were created by previous setup steps and should be spawned.
	pub tasks_to_spawn: Option<Vec<Box<dyn Future<Item = (), Error = ()> + Send>>>,
	inherent_data_providers: InherentDataProviders,
}

impl<F> Default for NodeConfig<F> where F: substrate_service::ServiceFactory {
	fn default() -> NodeConfig<F> {
		NodeConfig {
			import_setup: None,
			inherent_data_providers: InherentDataProviders::new(),
			tasks_to_spawn: None,
		}
	}
}

construct_service_factory! {
	struct Factory {
		Block = Block,
		RuntimeApi = RuntimeApi,
		NetworkProtocol = NodeProtocol { |config| Ok(NodeProtocol::new()) },
		RuntimeDispatch = stafi_executor::Executor,
		FullTransactionPoolApi = transaction_pool::ChainApi<client::Client<FullBackend<Self>, FullExecutor<Self>, Block, RuntimeApi>, Block>
			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
		LightTransactionPoolApi = transaction_pool::ChainApi<client::Client<LightBackend<Self>, LightExecutor<Self>, Block, RuntimeApi>, Block>
			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
		Genesis = GenesisConfig,
		Configuration = NodeConfig<Self>,
		FullService = FullComponents<Self>
			{ |config: FactoryFullConfiguration<Self>|
				FullComponents::<Factory>::new(config) },
		AuthoritySetup = {
			|mut service: Self::FullService| {
				let (block_import, link_half, babe_link) = service.config_mut().custom.import_setup.take()
					.expect("Link Half and Block Import are present for Full Services or setup failed before. qed");

				// spawn any futures that were created in the previous setup steps
				if let Some(tasks) = service.config_mut().custom.tasks_to_spawn.take() {
					for task in tasks {
						service.spawn_task(
							task.select(service.on_exit())
								.map(|_| ())
								.map_err(|_| ())
						);
					}
				}

				if service.config().roles.is_authority() {
					let proposer = substrate_basic_authorship::ProposerFactory {
						client: service.client(),
						transaction_pool: service.transaction_pool(),
					};

					let client = service.client();
					let select_chain = service.select_chain().ok_or(ServiceError::SelectChainRequired)?;

					let babe_config = babe::BabeParams {
						config: Config::get_or_compute(&*client)?,
						keystore: service.keystore(),
						client,
						select_chain,
						block_import,
						env: proposer,
						sync_oracle: service.network(),
						inherent_data_providers: service.config().custom.inherent_data_providers.clone(),
						force_authoring: service.config().force_authoring,
						time_source: babe_link,
					};

					let babe = start_babe(babe_config)?;
					let select = babe.select(service.on_exit()).then(|_| Ok(()));
					service.spawn_task(Box::new(select));
				}

				let config = grandpa::Config {
					// FIXME #1578 make this available through chainspec
					gossip_duration: Duration::from_millis(333),
					justification_period: 4096,
					name: Some(service.config().name.clone()),
					keystore: Some(service.keystore()),
				};

				match (service.config().roles.is_authority(), service.config().disable_grandpa) {
					(false, false) => {
						// start the lightweight GRANDPA observer
						service.spawn_task(Box::new(grandpa::run_grandpa_observer(
							config,
							link_half,
							service.network(),
							service.on_exit(),
						)?));
					},
					(true, false) => {
						// start the full GRANDPA voter
						let telemetry_on_connect = TelemetryOnConnect {
							telemetry_connection_sinks: service.telemetry_on_connect_stream(),
						};
						let grandpa_config = grandpa::GrandpaParams {
							config: config,
							link: link_half,
							network: service.network(),
							inherent_data_providers: service.config().custom.inherent_data_providers.clone(),
							on_exit: service.on_exit(),
							telemetry_on_connect: Some(telemetry_on_connect),
						};
						service.spawn_task(Box::new(grandpa::run_grandpa_voter(grandpa_config)?));
					},
					(_, true) => {
						grandpa::setup_disabled_grandpa(
							service.client(),
							&service.config().custom.inherent_data_providers,
							service.network(),
						)?;
					},
				}

				Ok(service)
			}
		},
		LightService = LightComponents<Self>
			{ |config| <LightComponents<Factory>>::new(config) },
		FullImportQueue = BabeImportQueue<Self::Block>
			{ |config: &mut FactoryFullConfiguration<Self> , client: Arc<FullClient<Self>>, select_chain: Self::SelectChain| {
				let (block_import, link_half) =
					grandpa::block_import::<_, _, _, RuntimeApi, FullClient<Self>, _>(
						client.clone(), client.clone(), select_chain
					)?;
				let justification_import = block_import.clone();

				let (import_queue, babe_link, babe_block_import, pruning_task) = import_queue(
					Config::get_or_compute(&*client)?,
					block_import,
					Some(Box::new(justification_import)),
					None,
					client.clone(),
					client,
					config.custom.inherent_data_providers.clone(),
				)?;

				config.custom.import_setup = Some((babe_block_import.clone(), link_half, babe_link));
				config.custom.tasks_to_spawn = Some(vec![Box::new(pruning_task)]);

				Ok(import_queue)
			}},
		LightImportQueue = BabeImportQueue<Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<LightClient<Self>>| {
				#[allow(deprecated)]
				let fetch_checker = client.backend().blockchain().fetcher()
					.upgrade()
					.map(|fetcher| fetcher.checker().clone())
					.ok_or_else(|| "Trying to start light import queue without active fetch checker")?;
				let block_import = grandpa::light_block_import::<_, _, _, RuntimeApi, LightClient<Self>>(
					client.clone(), Arc::new(fetch_checker), client.clone()
				)?;

				let finality_proof_import = block_import.clone();
				let finality_proof_request_builder = finality_proof_import.create_finality_proof_request_builder();

				// FIXME: pruning task isn't started since light client doesn't do `AuthoritySetup`.
				let (import_queue, ..) = import_queue(
					Config::get_or_compute(&*client)?,
					block_import,
					None,
					Some(Box::new(finality_proof_import)),
					client.clone(),
					client,
					config.custom.inherent_data_providers.clone(),
				)?;

				Ok((import_queue, finality_proof_request_builder))
			}},
		SelectChain = LongestChain<FullBackend<Self>, Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<FullClient<Self>>| {
				#[allow(deprecated)]
				Ok(LongestChain::new(client.backend().clone()))
			}
		},
		FinalityProofProvider = { |client: Arc<FullClient<Self>>| {
			Ok(Some(Arc::new(GrandpaFinalityProofProvider::new(client.clone(), client)) as _))
		}},
	}
}
