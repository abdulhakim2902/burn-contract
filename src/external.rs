use crate::*;

#[derive(Clone)]
#[near_sdk::near(serializers=[borsh, json])]
pub struct FungibleTokenMetadata {
	pub spec: String,
	pub name: String,
	pub symbol: String,
	pub icon: Option<String>,
	pub reference: Option<String>,
	pub reference_hash: Option<Base64VecU8>,
	pub decimals: u8,
}

#[ext_contract(ext_ft)]
trait WrappedAppchainToken {
	fn burn(&mut self, account_id: AccountId, amount: U128);
	fn ft_metadata() -> FungibleTokenMetadata;
}

#[ext_contract(ext_self)]
trait BurnResolver {
	fn resolve_burn(&mut self, token_id: AccountId, account_id: AccountId, amount: u128);
}
