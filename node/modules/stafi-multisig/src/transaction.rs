// Copyright 2018 Chainpool.

use super::{balances, system, Codec, Decode, Encode};
use rstd::prelude::*;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
    TransferChainX,
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::TransferChainX
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
pub struct Transaction {
    tx_type: TransactionType,
    data: Vec<u8>,
}

impl Transaction {
    pub fn new(tx_type: TransactionType, data: Vec<u8>) -> Self {
        Transaction { tx_type, data }
    }

    pub fn tx_type(&self) -> TransactionType {
        self.tx_type
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Transfer<AccountId, Balance>
where
    AccountId: Codec,
    Balance: Codec,
{
    pub to: AccountId,
    pub value: Balance,
}

#[allow(unused)]
pub type TransferT<T> = Transfer<<T as system::Trait>::AccountId, <T as balances::Trait>::Balance>;
