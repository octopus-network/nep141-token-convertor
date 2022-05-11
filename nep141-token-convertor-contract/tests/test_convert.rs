use crate::common::constant::{convertor_contract_id, string_to_account};
use crate::common::convertor::setup_pools;
use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::json;
use nep141_token_convertor_contract::token_receiver::ConvertAction;
use nep141_token_convertor_contract::token_receiver::TransferMessage::{AddLiquidity, Convert};

pub mod common;

#[allow(unused_variables)]
#[test]
pub fn test_convert() {
    let (root, admin, convertor, creator, user, whitelist_tokens, token_contracts) = setup_pools();
    convertor
        .create_pool(
            &creator,
            whitelist_tokens[0].token_id.clone(),
            whitelist_tokens[1].token_id.clone(),
            true,
            1,
            1,
            Option::None,
        )
        .assert_success();

    let token_in = &token_contracts[0];
    let token_out = &token_contracts[1];

    token_in
        .mint(&root, string_to_account("creator"), U128::from(100))
        .assert_success();

    token_in
        .ft_transfer_call(
            &creator,
            convertor_contract_id(),
            U128::from(10),
            Option::None,
            json!(AddLiquidity { pool_id: U64(1) }).to_string(),
        )
        .assert_success();

    // convertor.get_pools(0,10).get(0).unwrap()

    token_out
        .mint(&root, string_to_account("user"), U128::from(100))
        .assert_success();

    let convert_msg = json!(Convert {
        convert_action: ConvertAction {
            pool_id: U64(1),
            input_token_id: token_out.contract.account_id(),
            input_token_amount: U128::from(10)
        }
    })
    .to_string();

    token_out
        .ft_transfer_call(
            &user,
            convertor_contract_id(),
            U128::from(10),
            Option::None,
            convert_msg,
        )
        .assert_success();

    let user_token_in_balance = token_in.ft_balance_of(string_to_account("user")).0;
    println!(
        "user token in balance {}",
        token_in.ft_balance_of(string_to_account("user")).0
    );
    assert_eq!(
        10, user_token_in_balance,
        "user token balance should be 10."
    );
}
