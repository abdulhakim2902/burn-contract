use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
// use near_sdk::json_types::u128;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::store::LookupMap;
use near_sdk::{assert_one_yocto, env, ext_contract, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Gas};

#[ext_contract(ext_ft)]
trait WrappedAppchainToken {
    fn burn(&mut self, account_id: AccountId, amount: U128);
}

#[ext_contract(ext_self)]
trait BurnResolver {
    fn resolve_burn(&mut self, account_id: AccountId, amount: u128);
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub struct Account {
    amount: u128,
    session: u128,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
	AccountSession,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize,  PanicOnDefault)]
pub struct BurnToken {
    token_id: AccountId,
    accounts: LookupMap<AccountId, Account>,
}

#[near_bindgen]
impl BurnToken {
    #[init]
    pub fn new(token_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized.");
        Self { 
            token_id,
            accounts: LookupMap::new(StorageKeys::AccountSession),
        }
    }

    // view
    pub fn get_token_id(&self) -> AccountId {
        self.token_id.clone()
	}

    pub fn get_account_session(&self, account_id: &AccountId) -> Option<&Account> {
        self.accounts.get(account_id)
    }

    // call
    #[payable]
    pub fn burn(&mut self, amount: U128) {
        assert_one_yocto();
        ext_ft::ext(self.token_id.clone()).with_attached_deposit(env::attached_deposit()).with_static_gas(Gas::from_tgas(5)).burn(env::predecessor_account_id(), amount)
            .then(ext_self::ext(env::current_account_id()).with_static_gas(Gas::from_tgas(1)).resolve_burn(env::predecessor_account_id(), amount.into()));
    }

    pub fn set_token_id(&mut self, token_id: AccountId) {
        self.token_id = token_id;
    }

    #[private]
    pub fn resolve_burn(&mut self, account_id: AccountId, amount: u128) {
        let mut account = if let Some(account) = self.accounts.get(&account_id) {
            account.clone()
        } else {
            Account { amount: 0, session: 0 }
        };

        if let Some(new_balance) = account.amount.checked_add(amount) {
            account.amount = new_balance;
        } else {
            env::panic_str("Overflow account amount");
        }

        if let Some(new_balance) = account.session.checked_add(amount) {
            account.session = new_balance;
        } else {
            env::panic_str("Overflow account amount");
        }

        self.accounts.insert(account_id, account);
    }
}
