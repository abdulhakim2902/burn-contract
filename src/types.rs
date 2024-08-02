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
pub struct AccountKey(pub TokenId, pub AccountId);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
	AccountConversation,
}

#[near(serializers=[borsh, json])]
pub struct StorageBalance {
	pub total: NearToken,
	pub available: NearToken,
}
