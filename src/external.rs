use crate::*;

#[ext_contract(ext_ft)]
trait WrappedAppchainToken {
    fn burn(&mut self, account_id: AccountId, amount: U128);
}

#[ext_contract(ext_self)]
trait BurnResolver {
    fn resolve_burn(&mut self, token_id: AccountId, account_id: AccountId, amount: u128);
}