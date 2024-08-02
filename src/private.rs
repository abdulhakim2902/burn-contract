use crate::*;
use near_sdk::log;

#[near_bindgen]
impl Contract {
	#[private]
	pub fn resolve_burn(&mut self, account_id: AccountId) {
		require!(env::promise_results_count() == 2);

		let balance = match env::promise_result(0) {
			PromiseResult::Successful(val) => match from_slice::<StorageBalance>(&val) {
				Ok(storage) => storage.total,
				Err(_) => NearToken::from_near(0),
			},
			PromiseResult::Failed => NearToken::from_near(0),
		};

		log!("Available deposit {}", balance);

		let is_burned = match env::promise_result(1) {
			PromiseResult::Successful(_) => true,
			PromiseResult::Failed => false,
		};

		let burn_msg = if is_burned { "Token is burned" } else { "Failed to burn token" };

		log!("{}", burn_msg);

		Promise::new(account_id)
			.transfer(balance.saturating_add(NearToken::from_yoctonear(1)))
			.then(ext_self::ext(env::current_account_id()).resolve_transfer());
	}

	#[private]
	pub fn resolve_transfer(self) {
		require!(env::promise_results_count() == 1);

		let is_transferred = match env::promise_result(0) {
			PromiseResult::Successful(_) => true,
			PromiseResult::Failed => false,
		};

		let transfer_msg = if is_transferred { "Refund the deposit" } else { "Transfer failed" };

		log!("{}", transfer_msg);
	}
}
