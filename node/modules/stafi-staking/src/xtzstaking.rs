extern crate srml_system as system;

use srml_support::{decl_module, decl_storage, decl_event, StorageMap, ensure, dispatch::Result};
use system::ensure_signed;
use sr_std::prelude::*;
use sr_primitives::traits::Hash;
use parity_codec::{Encode, Decode};
use stafi_primitives::StakeTokenType;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum XtzStakeStage {
	// Init
	Init,
	// Transfer token to multi sig address
	Transfering,
	// Successful transfer
	TransferSuccess,
	// Active staking stage
	Staking,
	// Completed staking stage
	Completed,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq)]
pub struct XtzStakeTokenData {
	pub token_type: StakeTokenType,
	// decimals of token
	pub token_decimals: u32,
	// validator
	pub validator: Vec<u8>,
	// Amount of stake
	pub stake_amount: u128,
	// Reward of stake
	pub reward_amount: u128,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct XtzStakeData<AccountId, Hash> {
	// // identifier id
	pub id: Hash,
	// creator of stake
	pub initiator: AccountId,
	// multi sig address
	pub multi_sig_address: Vec<u8>,
	// Stage of stake
	pub stage: XtzStakeStage,
	// Token data of stake
	pub stake_token_data: XtzStakeTokenData,
}

pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		pub StakeRecords get(stake_records): map (T::AccountId, T::Hash) => Option<XtzStakeData<T::AccountId, T::Hash>>;
		pub StakeDataHashRecords get(stake_data_hash_records): map T::AccountId => Vec<T::Hash>;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// 
		pub fn custom_stake(origin, multi_sig_address: Vec<u8>, stake_amount: u128, validator: Vec<u8>, transfer_msg: Vec<u8>, signatures: Vec<u8>) -> Result {
			let sender = ensure_signed(origin)?;

			ensure!(stake_amount > 0, "Stake amount must be greater than 0");
			// // TODO: Check multi sig address
			// ensure!(stake_amount > 0, "Multi sig address is illegal");

			let random_seed = <system::Module<T>>::random_seed();
            let hash = (random_seed, &sender).using_encoded(<T as system::Trait>::Hashing::hash);

			let xtz_stake_token_data =  XtzStakeTokenData {
				token_type: StakeTokenType::XTZ,
				token_decimals: 18,
				validator: validator,
				stake_amount: stake_amount,
				reward_amount: 0,
			};

			<StakeRecords<T>>::insert((sender.clone(), hash.clone()), XtzStakeData {
				id: hash.clone(),
				initiator: sender.clone(),
				multi_sig_address: multi_sig_address,
				stage: XtzStakeStage::Init,
				stake_token_data: xtz_stake_token_data.clone(),
			});

			let mut hashs = <StakeDataHashRecords<T>>::get(sender.clone());
			hashs.push(hash.clone());
			<StakeDataHashRecords<T>>::insert(sender.clone(), hashs);

			// here we are raising the event
			Self::deposit_event(RawEvent::StakeTransfer(sender, hash));
			Ok(())
		}

		// 
		// pub fn stake(origin, something: u32) -> Result {
		// 	// TODO: You only need this if you want to check it was signed.
		// 	let who = ensure_signed(origin)?;

		// 	// TODO: Code to execute when something calls this.
		// 	// For example: the following line stores the passed in u32 in the storage
		// 	Something::put(something);

		// 	// here we are raising the Something event
		// 	Self::deposit_event(RawEvent::SomethingStored(something, who));
		// 	Ok(())
		// }
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Hash = <T as system::Trait>::Hash {
		StakeTransfer(AccountId, Hash),
	}
);

