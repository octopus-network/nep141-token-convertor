use near_sdk::AccountId;
use near_sdk_sim::{
    call, deploy, init_simulator, to_yocto, view, ContractAccount, ExecutionResult, UserAccount,
};
use nep141_token_convertor_contract::TokenConvertorContract;

// pub fn deploy_prize_pool_contract(signer_account: &UserAccount, admin: Option<AccountId>)->ContractAccount<>
