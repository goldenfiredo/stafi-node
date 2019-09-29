#![cfg_attr(not(feature = "std"), no_std)]
extern crate session;
extern crate sr_primitives as runtime_primitives;
extern crate srml_balances as balances;
extern crate srml_support as support;
extern crate srml_system as system;

use log::info;
use sr_primitives::traits::Member;
use substrate_application_crypto::AppPublic;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, Parameter, StorageValue};
use system::ensure_signed;
use sr_primitives::traits::CheckedAdd;
use sr_std::{
 convert::{TryInto},
};

pub trait Trait: system::Trait + session::Trait {
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
        let free_balance = <balances::Module<T>>::free_balance(account_id.clone());
        let add_value: Balance = 10 * 1_000_000_000 * 1_000 * 1_000;
        if let Some(value) = add_value.try_into().ok() {
            // check
            match free_balance.checked_add(&value) {
                Some(b) => balances::FreeBalance::<T>::insert(&account_id.clone(), b),
                None => (),
            };
        }
    }
}
