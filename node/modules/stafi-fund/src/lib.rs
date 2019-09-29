#![cfg_attr(not(feature = "std"), no_std)]
extern crate session;
extern crate sr_primitives;
extern crate srml_balances as balances;
extern crate srml_support as support;
extern crate srml_system as system;

use hex_literal::hex;
use log::info;
use sr_primitives::traits::Member;
use substrate_application_crypto::AppPublic;
use support::{decl_event, StorageMap, decl_module, decl_storage, dispatch::Result, Parameter, StorageValue};
use system::ensure_signed;
use stafi_primitives::{ Balance, AccountId}; 
use sr_primitives::{traits::{Verify, CheckedAdd},AnySignature};
use substrate_primitives::crypto::UncheckedInto;
use substrate_primitives::sr25519::{Signature, Public};
use parity_codec::{Encode, Decode};


use sr_std::{
 convert::{TryInto},
};

pub type AnySig = AnySignature;

pub trait Trait: system::Trait + session::Trait + balances::Trait{
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type AuthorityId: Member + Parameter + AppPublic + Default;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Something get(something): Option<u32>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		fn deposit_event() = default;

		pub fn do_something(origin, something: u32) -> Result {
			let who = ensure_signed(origin)?;
			Something::put(something);
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		SomethingStored(u32, AccountId),
	}
);

impl<T: Trait> session::OneSessionHandler<T::AccountId> for Module<T> {
	type Key = T::AuthorityId;
	fn on_new_session<'a, I: 'a>(changed: bool, validators: I, _queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{


		// let a = AnySig::from("5a9755f069939f45d96aaf125cf5ce7ba1db998686f87f2fb3cbdea922078741a73891ba265f70c31436e18a9acd14d189d73c12317ab6c313285cd938453202");
		// info!("11111111111{}", a);
		// //let account_id:AccountId = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
		// //let account_id = AccountKeyring::Alice.into();
		// let signId:Signature = Signature::from_raw(hex!["5a9755f069939f45d96aaf125cf5ce7ba1db998686f87f2fb3cbdea922078741a73891ba265f70c31436e18a9acd14d189d73c12317ab6c313285cd938453202"]);
		// let pubid:AccountId = Public(hex!["982e1ff9bc6e2c1b04b2d66a04f28aff2f767d330aefc43ffecc7d912b29727f"]);
		// //let account_id:AccountId = hex!["d43b38b84b60b06e7f1a00d892dcff67ea69dc1dc2f837fdb6a27344b63c9279"].unchecked_into();
		// // let aa = LocalizedSignature{
		// // 	signer:pubid,
		// // 	signature:signId,
		// // };

		// //let account_id:AccountId = AnySig::from(signId);
		// let AnySignatureId:AnySig = AnySig("5a9755f069939f45d96aaf125cf5ce7ba1db998686f87f2fb3cbdea922078741a73891ba265f70c31436e18a9acd14d189d73c12317ab6c313285cd938453202");
		// let account_id:AccountId = Verify::Signer;

		
	}

	fn on_before_session_ending() {
		let a: Public = hex!["d43b38b84b60b06e7f1a00d892dcff67ea69dc1dc2f837fdb6a27344b63c9279"].unchecked_into();
		let account_id: T::AccountId = a.using_encoded(|mut s| Decode::decode(&mut s)).expect("Panic");

		let free_balance = <balances::Module<T>>::free_balance::<T::AccountId>(account_id.clone());
		let add_value: Balance = 10 * 1_000_000_000 * 1_000 * 1_000;
		if let Some(value) = add_value.try_into().ok() {
			// check
			match free_balance.checked_add(&value) {
				Some(b) => balances::FreeBalance::<T>::insert::<T::AccountId, T::Balance>(account_id.clone(), b),
				None => (),
			};
		}
	}

	fn on_genesis_session<'a, I: 'a>(validators: I)
		where I: Iterator<Item=(&'a T::AccountId, T::AuthorityId)>
	{

	}

	fn on_disabled(i: usize) {
		
	}

}
