mod external;
mod ft_callback;
mod private;
mod types;

use near_sdk::{
	borsh::{BorshDeserialize, BorshSerialize},
	env, ext_contract,
	json_types::{Base64VecU8, U128},
	near, near_bindgen, require,
	serde::{Deserialize, Serialize},
	serde_json::from_slice,
	store::LookupMap,
	AccountId, BorshStorageKey, Gas, NearToken, PanicOnDefault, Promise, PromiseOrValue,
	PromiseResult,
};

use external::*;
use types::*;

#[derive(
	BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy,
)]
pub struct Conversation {
	amount: u128,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
	accounts: LookupMap<AccountKey, Conversation>,
}

#[near_bindgen]
impl Contract {
	#[init]
	pub fn new() -> Self {
		assert!(!env::state_exists(), "Already initialized.");

		Self { accounts: LookupMap::new(StorageKeys::AccountConversation) }
	}

	// view
	pub fn balance_of(&self, token_id: TokenId, account_id: AccountId) -> u128 {
		match self.accounts.get(&AccountKey(token_id, account_id)) {
			Some(conversation) => conversation.amount,
			None => 0,
		}
	}

	// call
	pub fn converse(&mut self, token_id: TokenId, amount: U128) -> Conversation {
		let account_id = env::signer_account_id();
		let account_key = AccountKey(token_id, account_id);

		match self.accounts.get(&account_key) {
			Some(conversation) => {
				require!(conversation.amount >= amount.into(), "Insufficient conversation amount");

				let mut conversation = conversation.to_owned();

				conversation.amount = conversation
					.amount
					.checked_sub(amount.into())
					.expect("Overflow conversation amount");

				self.accounts.insert(account_key, conversation);

				conversation
			},
			None => {
				env::panic_str("Account not found");
			},
		}
	}

	#[payable]
	pub fn burn(&mut self, token_id: TokenId) {
		let account_id = env::signer_account_id();

		ext_ft::ext(token_id.clone())
			.storage_balance_of(account_id.clone())
			.and(
				ext_ft::ext(token_id.clone())
					.with_attached_deposit(NearToken::from_yoctonear(1))
					.with_static_gas(Gas::from_tgas(1))
					.storage_unregister(Some(true)),
			)
			.then(
				ext_self::ext(env::current_account_id())
					.with_static_gas(Gas::from_tgas(1))
					.resolve_burn(account_id),
			);
	}
}
