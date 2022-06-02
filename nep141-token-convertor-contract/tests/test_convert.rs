use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*;

use crate::common::constant::CONVERTOR_CONTRACT;
use crate::common::utils::{setup_convertor_contract_roles, setup_pools};
use nep141_token_convertor_contract::token_receiver::TransferMessage::{AddLiquidity, Convert};
use nep141_token_convertor_contract::token_receiver::ConvertAction;

mod common;

#[tokio::test]
pub async fn test_convert() {
    let (  whitelist_tokens, token_contracts) = setup_pools().await;
    let (root, owner, creator, user) = setup_convertor_contract_roles().await;
    let convertor_contract = CONVERTOR_CONTRACT.get().await;

    convertor_contract
        .create_pool(
            &creator,
            whitelist_tokens[0].token_id.clone(),
            whitelist_tokens[1].token_id.clone(),
            true,
            1,
            1,
            Some(parse_near!("1 N")),
        ).await.unwrap();

    let token_in = &token_contracts[0];
    let token_out = &token_contracts[1];

    token_in
        .mint( creator.id().clone(), U128::from(100))
        .await.unwrap();

    token_in
        .ft_transfer_call(
            &creator,
            convertor_contract.contract_id.clone(),
            U128::from(10),
            Option::None,
            json!(AddLiquidity { pool_id: U64(1) }).to_string(),
        ).await.unwrap();

    // convertor.get_pools(0,10).get(0).unwrap()

    token_out
        .mint( user.id().clone(), U128::from(100))
        .await.unwrap();

    let convert_msg = json!(Convert {
        convert_action: ConvertAction {
            pool_id: U64(1),
            input_token_id: near_sdk::AccountId::new_unchecked(token_out.contract_id.to_string()),
            input_token_amount: U128::from(10)
        }
    })
    .to_string();

    token_out
        .ft_transfer_call(
            &user,
            convertor_contract.contract_id.clone(),
            U128::from(10),
            Option::None,
            convert_msg,
        ).await.unwrap();

    let user_token_in_balance = token_in.ft_balance_of(user.id().clone()).await.0;
    println!(
        "user token in balance {}",
        token_in.ft_balance_of(user.id().clone()).await.0
    );
    assert_eq!(
        10, user_token_in_balance,
        "user token balance should be 10."
    );
}
