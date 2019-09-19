#![cfg_attr(not(feature = "std"), no_std)]

extern crate srml_support as support;
extern crate srml_system as system;
extern crate srml_balances as balances;
extern crate sr_primitives as runtime_primitives;

use support::{decl_module, decl_storage, decl_event, StorageMap, dispatch::Result, Parameter, dispatch::Vec};
use system::ensure_signed;
use parity_codec::{Codec, Encode, Decode};
use sr_primitives::traits::MaybeSerializeDebug;
use runtime_primitives::traits::Hash;
use stafi_primitives::{ Balance}; 
use srml_timestamp as timestamp;

pub type SymbolString = &'static [u8];
pub type DescString = SymbolString;

pub trait Trait: balances::Trait+timestamp::Trait {
	const STAFI_SYMBOL: SymbolString;
    const STAFI_TOKEN_DESC: DescString;
	type TokenBalance: Parameter + Codec + Default + Copy + MaybeSerializeDebug + From<Self::BlockNumber>;
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
	XTZ,
	ATOM,
}
impl Default for Symbol {
	fn default() -> Symbol {
		Symbol::XTZ
	}
}

pub type TokenDesc = Vec<u8>;
pub type Precision = u16;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive( Debug))]
pub struct Token {
    symbol: Symbol,
    token_desc: TokenDesc,
    precision: Precision,
}

pub type RewardsAmount = u16;
//pub type AccountId = Vec<u8>;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive( Debug))]
pub struct BondToken<Moment, AccountId, Hash> {
	symbol: Symbol,
	balance: Balance,
	rewards_amount: RewardsAmount,
	account_id: AccountId,
	staking_time: Moment,
	hash: Hash,
}

impl Token {
    pub fn new(symbol: Symbol, token_desc: TokenDesc, precision: Precision) -> Self {
        Token {
            symbol,
            token_desc,
            precision,
        }
    }

    pub fn symbol(&self) -> Symbol {
        self.symbol.clone()
    }

    pub fn precision(&self) -> Precision {
        self.precision
    }

    pub fn token_desc(&self) -> TokenDesc {
        self.token_desc.clone()
    }

    pub fn set_token_desc(&mut self, desc: &TokenDesc) {
        self.token_desc = desc.clone();
    }

}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		pub TokenInfo get(token_info): map Symbol => Token;

        pub TotalFreeToken get(total_free_token): map Symbol => T::TokenBalance;

        pub FreeToken get(token_free_balance): map (T::AccountId, Symbol) => T::TokenBalance;

		pub FreeBondToken get(bond_token_free_balance): map (T::AccountId, T::Hash) => BondToken<T::Moment, T::AccountId, T::Hash>;
		
		pub BondTokenHash get(bond_token_hash): map T::AccountId => Option<(T::Hash, Symbol)>;
        //pub TokenListOf get(token_list_of): map T::AccountId => Vec<Symbol> = [T::STAFI_SYMBOL.to_vec()].to_vec();
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn register_token(
            origin,
            symbol: Symbol,
            token_desc: TokenDesc,
            precision: Precision
        ) -> Result {
			let who = ensure_signed(origin)?;
            let token = Token{
				symbol : symbol,
            	token_desc : token_desc,
            	precision : precision,
			};
            <TokenInfo>::insert(token.symbol(), token.clone());
			Self::deposit_event(RawEvent::TokenInfoStored(token, who));
            Ok(())
        }

		pub fn set_free_token(
            origin, 
            who: T::AccountId, 
            sym: Symbol, 
            free: T::TokenBalance
            ) -> Result {
            let from = ensure_signed(origin)?;
			let key = (who.clone(), sym.clone());
            FreeToken::<T>::insert(key, free);
			Self::deposit_event(RawEvent::FreeTokenStored(sym.clone(), from));
            Ok(())
        }

		pub fn set_free_bond_token(
            origin, 
            sym: Symbol, 
            free: Balance
            ) -> Result {
            let who = ensure_signed(origin)?;
			Self::add_bond_token(who.clone(), sym, free);
			Self::deposit_event(RawEvent::FreeBondTokenStored(sym.clone(), who));
			Ok(())
        }
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
		SomeValueStored(u32, AccountId),
		TokenInfoStored(Token, AccountId),
		FreeTokenStored(Symbol, AccountId),
		FreeBondTokenStored(Symbol, AccountId),
	}
);

impl<T: Trait> Module<T> {
	fn add_bond_token(who: T::AccountId, symbol: Symbol, free: Balance) -> Result {
        	let random_seed = <system::Module<T>>::random_seed();
            let hash = (random_seed, &who).using_encoded(<T as system::Trait>::Hashing::hash);
			let key = (who.clone(), hash.clone());
			let _now = <timestamp::Module<T>>::get();
			let bond_tokne = BondToken{
				symbol: symbol,
				balance: free,
				rewards_amount: 0,
				account_id: who.clone(),
				staking_time: _now,
				hash: hash,
			};
			BondTokenHash::<T>::insert(who.clone(), (hash.clone(), symbol.clone()));
            FreeBondToken::<T>::insert(key, bond_tokne);
        	Ok(())
    }


}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}