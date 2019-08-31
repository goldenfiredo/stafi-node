extern crate srml_system as system;

use srml_support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, ensure, dispatch::Result};
use system::ensure_signed;
use sr_std::prelude::*;
use sr_primitives::traits::Hash;
use parity_codec::{Encode, Decode};
use stafi_primitives::StakeTokenType;


#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum AtomStakeStage {
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

// TODO 
// put this struct into specific module
// #[cfg_attr(feature = "std", derive(Debug))]
// #[derive(Encode, Decode, PartialEq)]
// pub struct MultiSigAddress<Hash> {
// 	// public key
// 	pub public_key: Hash,
// 	// multi sig address
// 	pub address: Hash,
// }

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq)]
pub struct AtomStakeTokenData {
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
pub struct AtomStakeData<AccountId, Hash> {
	// // identifier id
	pub id: Hash,
	// creator of stake
	pub initiator: AccountId,
	// multi sig address
	pub multi_sig_address: Vec<u8>,
	// Stage of stake
	pub stage: AtomStakeStage,
	// Token data of stake
	pub stake_token_data: AtomStakeTokenData,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq)]
pub struct AtomTransferData<AccountId, Hash> {
	// // identifier id
	pub id: Hash,
	// creator of stake
	pub initiator: AccountId,
	// multi sig address
	pub transfer_msg: Vec<u8>,
	// Stage of stake
	pub signatures: Vec<u8>,
}

pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		pub StakeRecords get(stake_records): map (T::AccountId, T::Hash) => Option<AtomStakeData<T::AccountId, T::Hash>>;
		pub StakeDataHashRecords get(stake_data_hash_records): map T::AccountId => Vec<T::Hash>;
		pub TransferDataQueue get(transfer_data_queue): Vec<AtomTransferData<T::AccountId, T::Hash>>;
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

			let stake_token_data = AtomStakeTokenData {
				token_type: StakeTokenType::ATOM,
				token_decimals: 18,
				validator: validator,
				stake_amount: stake_amount,
				reward_amount: 0,
			};

			<StakeRecords<T>>::insert((sender.clone(), hash.clone()), AtomStakeData {
				id: hash.clone(),
				initiator: sender.clone(),
				multi_sig_address: multi_sig_address,
				stage: AtomStakeStage::Init,
				stake_token_data: stake_token_data.clone(),
			});

			let mut hashs = <StakeDataHashRecords<T>>::get(sender.clone());
			hashs.push(hash.clone());
			<StakeDataHashRecords<T>>::insert(sender.clone(), hashs);

			let transfer_data =  AtomTransferData {
				id: hash.clone(),
				initiator: sender.clone(),
				transfer_msg: transfer_msg,
				signatures: signatures,
			};
			let mut queue = Self::transfer_data_queue();
			queue.push(transfer_data.clone());
			<TransferDataQueue<T>>::put(queue);

			// here we are raising the event
			Self::deposit_event(RawEvent::StakeInit(sender, hash));
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
		StakeInit(AccountId, Hash),
	}
);

