use crate::*;

trait FungibleTokenReceiver {
	fn ft_on_transfer(
		&mut self,
		sender_id: AccountId,
		amount: U128,
		msg: String,
	) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
	fn ft_on_transfer(
		&mut self,
		sender_id: AccountId,
		amount: U128,
		msg: String,
	) -> PromiseOrValue<U128> {
		let token_id = env::predecessor_account_id();
		let account_key = AccountKey(token_id, sender_id);

		let mut conversation = match self.accounts.get(&account_key) {
			Some(conversation) => conversation.to_owned(),
			None => Conversation { amount: 0 },
		};

		conversation.amount =
			conversation.amount.checked_add(amount.into()).expect("Overflow account amount");

		self.accounts.insert(account_key, conversation);

		PromiseOrValue::Value(U128(0))
	}
}
