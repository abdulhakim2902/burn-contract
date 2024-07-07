use crate::*;

#[near_bindgen]
impl Contract {
	#[private]
	pub fn resolve_burn(&mut self, token_id: AccountId, account_id: AccountId, amount: u128) {
		require!(env::promise_results_count() == 2);

		let is_burned = match env::promise_result(0) {
			PromiseResult::Successful(_) => true,
			PromiseResult::Failed => false,
		};

		require!(is_burned, "Failed to burn token");

		let token_decimals = match env::promise_result(1) {
			PromiseResult::Successful(val) => match from_slice::<FungibleTokenMetadata>(&val) {
				Ok(ft_metadata) => ft_metadata.decimals,
				Err(_) => 18,
			},
			PromiseResult::Failed => 18,
		};

		let session_key = SessionKey(token_id, account_id);
		let mut account = match self.accounts.get(&session_key) {
			Some(account) => account.to_owned(),
			None => Account { burn_amount: 0, session: 0 },
		};

		let converted_session = amount
			.checked_div(u128::pow(10, token_decimals.into()))
			.expect("Invalid division");
		let burn_amount = account.burn_amount.checked_add(amount).expect("Overflow token amount");
		let total_session = account
			.session
			.checked_add(converted_session)
			.expect("Overflow account session");

		account.burn_amount = burn_amount;
		account.session = total_session;

		self.accounts.insert(session_key, account);
	}
}
