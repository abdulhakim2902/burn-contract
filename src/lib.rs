mod types;
mod tests;
mod private;
mod external;

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::store::LookupMap;
use near_sdk::{assert_one_yocto, env, ext_contract, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Gas};

use external::*;
use types::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub struct Account {
    burn_amount: u128,
    session: u128,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize,  PanicOnDefault)]
pub struct Contract {
    accounts: LookupMap<SessionKey, Account>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized.");

        Self {
            accounts: LookupMap::new(StorageKeys::AccountSession),
        }
    }

    // view
    pub fn get_account_session(&self, token_id: AccountId, account_id: AccountId) -> &Account {
        self.accounts.get(&SessionKey(token_id, account_id)).unwrap_or(&Account { burn_amount: 0, session: 0 })
    }

    // call
    #[payable]
    pub fn burn(&mut self, token_id: AccountId, amount: U128) {
        assert_one_yocto();

        ext_ft::ext(token_id.clone())
            .with_attached_deposit(env::attached_deposit())
            .with_static_gas(Gas::from_tgas(1))
            .burn(env::signer_account_id(), amount)
            .then(ext_self::ext(env::current_account_id())
            .with_static_gas(Gas::from_tgas(1))
            .resolve_burn(token_id, env::signer_account_id(), amount.into()));
    }
}
