use near_sdk_sim::{call, ContractAccount, view};
use crate::common::convertor::setup_pools;
use test_token::ContractContract as TestTokenContract;
use nep141_token_convertor_contract::token_receiver::TransferMessage::{AddLiquidity, Convert};
use near_sdk::serde_json::json;
use crate::common::constant::{convertor_contract_id, string_to_account};
use near_sdk::json_types::U128;
use nep141_token_convertor_contract::token_receiver::ConvertAction;


pub mod common;

#[test]
pub fn test_convert() {
    let (root, admin, convertor, creator, user, whitelist_tokens, token_contracts) = setup_pools();
    convertor.create_pool(
        &creator,
        whitelist_tokens[0].token_id.clone(),
        whitelist_tokens[1].token_id.clone(),
        true,
        1, 1, Option::None,
    ).assert_success();

    let token_in: &ContractAccount<TestTokenContract> = &token_contracts[0];
    let token_out: &ContractAccount<TestTokenContract> = &token_contracts[1];
    call!(
        root,
        token_in.mint(string_to_account("creator"), U128::from(100))
    ).assert_success();

    call!(
        creator,
        token_in.ft_transfer_call(
            convertor_contract_id(),
            U128::from(10),
            Option::None,
            json!(AddLiquidity { pool_id: 1 }).to_string()
        ),
        deposit = 1
    ).assert_success();

    call!(
        root,
        token_out.mint(string_to_account("user"), U128::from(100))
    ).assert_success();
    let convert_msg = json!(Convert { convert_action: ConvertAction {
        pool_id: 1,
        input_token_id: token_out.account_id(),
        input_token_amount: U128::from(10)
    } }).to_string();
    call!(user, token_out.ft_transfer_call(
        convertor_contract_id(),
        U128::from(10),Option::None,
        convert_msg),deposit=1).assert_success();
    let user_token_in_balance = view!(token_in.ft_balance_of(string_to_account("user"))).unwrap_json::<U128>();
    assert_eq!(10,user_token_in_balance.0);
}