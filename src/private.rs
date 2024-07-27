use crate::*;

#[near_bindgen]
impl Contract {
	#[private]
	pub fn resolve_burn(&mut self, token_id: TokenId, account_id: AccountId, amount: u128) {
		require!(env::promise_results_count() == 2);

		// Check is token is successfully burned
		let is_burned = match env::promise_result(0) {
			PromiseResult::Successful(_) => true,
			PromiseResult::Failed => false,
		};

		require!(is_burned, "Failed to burn token");

		// // Get the token decimal
		// let token_decimals = match env::promise_result(1) {
		// 	PromiseResult::Successful(val) => match from_slice::<FungibleTokenMetadata>(&val) {
		// 		Ok(ft_metadata) => ft_metadata.decimals,
		// 		Err(_) => 18,
		// 	},
		// 	PromiseResult::Failed => 18,
		// };

		let account_key = AccountKey(token_id, account_id);
		let mut conversation = match self.accounts.get(&account_key) {
			Some(conversation) => conversation.to_owned(),
			None => Conversation { amount: 0 },
		};

		// // Convert token amount to conversation amount
		// let conversation_amount = amount
		// 	.checked_div(u128::pow(10, token_decimals.into()))
		// 	.expect("Invalid division");

		conversation.amount =
			conversation.amount.checked_add(amount).expect("Overflow account amount");

		self.accounts.insert(account_key, conversation);
	}
}
