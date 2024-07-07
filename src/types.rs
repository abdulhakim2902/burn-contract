use crate::*;

pub type TokenId = AccountId;

#[derive(
	BorshDeserialize,
	BorshSerialize,
	Serialize,
	Deserialize,
	Clone,
	Debug,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
#[serde(crate = "near_sdk::serde")]
pub struct SessionKey(pub TokenId, pub AccountId);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
	AccountSession,
}
