use parity_codec::{Encode, Decode};


#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum StakeTokenType {
	// stafi token
	FIS,
	// tezos token
	XTZ,
	// cosmos token
	ATOM
}