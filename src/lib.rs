mod external;
mod private;
mod test;
mod types;

use near_sdk::{
	assert_one_yocto,
	borsh::{BorshDeserialize, BorshSerialize},
	env, ext_contract,
	json_types::{Base64VecU8, U128},
	near_bindgen, require,
	serde::{Deserialize, Serialize},
	serde_json::from_slice,
	store::LookupMap,
	AccountId, BorshStorageKey, Gas, PanicOnDefault, PromiseResult,
};

use external::*;
use types::*;

#[derive(
	BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy,
)]
pub struct Account {
	burn_amount: u128,
	session: u128,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
	accounts: LookupMap<SessionKey, Account>,
}

#[near_bindgen]
impl Contract {
	#[init]
	pub fn new() -> Self {
		assert!(!env::state_exists(), "Already initialized.");

		Self { accounts: LookupMap::new(StorageKeys::AccountSession) }
	}

	// view
	pub fn get_account_session(&self, token_id: AccountId, account_id: AccountId) -> &Account {
		self.accounts
			.get(&SessionKey(token_id, account_id))
			.unwrap_or(&Account { burn_amount: 0, session: 0 })
	}

	// call
	pub fn use_session(&mut self, token_id: AccountId, amount: U128) -> Account {
		let account_id = env::signer_account_id();
		let session_key = SessionKey(token_id, account_id);

		match self.accounts.get(&session_key) {
			Some(account) => {
				require!(account.session >= amount.into(), "Insufficient session amount");

				let new_session =
					account.session.checked_sub(amount.into()).expect("Overflow session amount");
				let mut account = account.to_owned();

				account.session = new_session;
				account
			},
			None => {
				env::panic_str("Account not found");
			},
		}
	}

	#[payable]
	pub fn burn(&mut self, token_id: AccountId, amount: U128) {
		assert_one_yocto();

		ext_ft::ext(token_id.clone())
			.with_attached_deposit(env::attached_deposit())
			.with_static_gas(Gas::from_tgas(1))
			.burn(env::signer_account_id(), amount)
			.and(ext_ft::ext(token_id.clone()).ft_metadata())
			.then(
				ext_self::ext(env::current_account_id())
					.with_static_gas(Gas::from_tgas(1))
					.resolve_burn(token_id, env::signer_account_id(), amount.into()),
			);
	}
}
