use crate::*;
#[ext_contract(ext_self)]
pub trait TokenConvertor {
    fn ft_transfer_resolved(&mut self, token_id: AccountId, sender_id: AccountId, amount: U128);
}
