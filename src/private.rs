use crate::*;

#[near_bindgen]
impl Contract {
  #[private]
  pub fn resolve_burn(&mut self, token_id: AccountId, account_id: AccountId, amount: u128) {
    require!(env::promise_results_count() == 1);

    if let PromiseResult::Successful(_) = env::promise_result(0) {
        let session_key = SessionKey(token_id, account_id);
        let mut account = if let Some(account) = self.accounts.get(&session_key) {
            account.clone()
        } else {
            Account { burn_amount: 0, session: 0 }
        };
    
        if let Some(new_balance) = account.burn_amount.checked_add(amount) {
            account.burn_amount = new_balance;
        } else {
            env::panic_str("Overflow account amount");
        }
    
        if let Some(new_session) = amount.checked_div(u128::pow(10, 18)) {
            if let Some(total_session) = account.session.checked_add(new_session) {
                account.session = total_session;
            } else {
                env::panic_str("Overflow account session");
            }
        } else {
            env::panic_str("Invalid division");
        }
    
        self.accounts.insert(session_key, account);
    }
    
  }
}