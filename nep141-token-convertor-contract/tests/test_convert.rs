use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*;

use crate::common::utils::setup_pools;
use nep141_token_convertor_contract::token_receiver::TransferMessage::{AddLiquidity, Convert};
use nep141_token_convertor_contract::token_receiver::ConvertAction;

mod common;

#[tokio::test]
pub async fn test_convert() {

    let (  worker, whitelist_tokens, token_contracts, convertor_contract, root, owner, creator, user ) = setup_pools().await;

    convertor_contract
        .create_pool(
            &worker,
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
        .mint( &worker, creator.id().clone(), U128::from(100))
        .await.unwrap();

    token_in
        .ft_transfer_call(
            &worker,
            &creator,
            convertor_contract.contract_id.clone(),
            U128::from(10),
            Option::None,
            json!(AddLiquidity { pool_id: U64(1) }).to_string(),
        ).await.unwrap();

    token_out
        .mint( &worker, user.id().clone(), U128::from(100))
        .await.unwrap();

    let convert_msg = json!(Convert {
        convert_action: ConvertAction {
            pool_id: U64(1),
            input_token_id: near_sdk::AccountId::new_unchecked(token_out.contract_id.to_string()),
            input_token_amount: U128::from(10)
        }
    })
    .to_string();

    let result =  token_out
        .ft_transfer_call(
            &worker,
            &user,
            convertor_contract.contract_id.clone(),
            U128::from(10),
            Option::None,
            convert_msg,
        ).await.unwrap().is_success();
    println!("{:?}", result);


    let user_token_in_balance = token_in.ft_balance_of(&worker,user.id().clone()).await.0;

    let a =  convertor_contract.get_pools(&worker, 0, 10).await[0].clone();
    println!("{:?}", a);

    assert_eq!(
        10, user_token_in_balance,
        "user token balance should be 10."
    );
}
