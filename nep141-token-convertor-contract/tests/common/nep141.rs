use crate::common::contracts::print_execution_result;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk_sim::{call, view, ContractAccount, ExecutionResult, UserAccount};
use test_token::ContractContract as TestTokenContract;

pub struct Nep141 {
    pub contract: ContractAccount<TestTokenContract>,
}

impl Nep141 {
    pub fn ft_transfer_call(
        &self,
        signer: &UserAccount,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> ExecutionResult {
        let contract = &self.contract;
        let result = call!(
            signer,
            contract.ft_transfer_call(receiver_id, amount, memo, msg),
            deposit = 1
        );
        print_execution_result(&result);
        result
    }

    pub fn mint(
        &self,
        signer: &UserAccount,
        account_id: AccountId,
        amount: U128,
    ) -> ExecutionResult {
        let contract = &self.contract;
        let result = call!(signer, contract.mint(account_id, amount));
        print_execution_result(&result);
        result
    }

    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        let contract = &self.contract;
        view!(contract.ft_balance_of(account_id)).unwrap_json::<U128>()
    }
}
