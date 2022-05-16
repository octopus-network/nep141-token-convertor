use crate::common::constant::*;
use crate::common::convertor::Convertor;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk_sim::{
    call, deploy, init_simulator, to_yocto, ContractAccount, ExecutionResult, UserAccount,
};
use nep141_token_convertor_contract::TokenConvertorContract;
use test_token::ContractContract as TestTokenContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONVERTOR_WASM_BYTES => "../res/nep141_token_convertor_contract.wasm",
    TEST_TOKEN_WASM_BYTES => "../res/test_token.wasm",
}

pub fn setup_convertor_contract() -> (UserAccount, UserAccount, Convertor) {
    let root = init_simulator(None);
    let owner = root.create_user(string_to_account("owner"), to_yocto("100"));
    let convertor = Convertor {
        contract: deploy_convertor_contract(&root, owner.account_id.clone(), U128(to_yocto("0"))),
    };
    return (root, owner, convertor);
}

pub trait NearContract<T> {
    fn get_contract(&self) -> &ContractAccount<T>;
}

pub fn should_failed(result: &ExecutionResult) {
    assert!(!result.is_ok(), "should failed");
}

pub fn print_execution_result(result: &ExecutionResult) {
    let results = result.promise_results();
    for sub_result in results {
        if let Some(sub_result) = sub_result {
            if sub_result.is_ok() {
                let logs = sub_result.logs();
                if logs.len() > 0 {
                    println!("{:#?}", logs);
                }
            } else {
                println!("{:#?}", sub_result.outcome());
            }
        }
    }
    if result.is_ok() {
        let logs = result.logs();
        if logs.len() > 0 {
            println!("{:#?}", logs);
        }
    } else {
        println!("{:#?}", result.outcome());
    }
}

pub fn deploy_convertor_contract(
    signer_account: &UserAccount,
    owner: AccountId,
    deposit_near_amount: U128,
) -> ContractAccount<TokenConvertorContract> {
    let contract = deploy! {
        contract: TokenConvertorContract,
        contract_id: convertor_contract_id(),
        bytes: &CONVERTOR_WASM_BYTES,
        signer_account: signer_account,
        init_method: new(owner_id(),deposit_near_amount)
    };
    contract
}

pub fn deploy_test_token_contract(
    signer_account: &UserAccount,
    token_id: AccountId,
    accounts_to_register: Vec<AccountId>,
) -> ContractAccount<TestTokenContract> {
    let t = deploy!(
        contract: TestTokenContract,
        contract_id: token_id,
        bytes: &TEST_TOKEN_WASM_BYTES,
        signer_account: signer_account,
        deposit: to_yocto("5"),
        init_method: new()
    );
    call!(
        signer_account,
        t.mint(signer_account.account_id.clone(), to_yocto("1000").into())
    )
    .assert_success();
    for account_id in accounts_to_register {
        call!(
            signer_account,
            t.storage_deposit(Some(account_id), None),
            deposit = to_yocto("1")
        )
        .assert_success();
    }
    t
}
