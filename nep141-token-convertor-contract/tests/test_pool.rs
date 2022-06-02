use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*;

use crate::common::constant::CONVERTOR_CONTRACT;
use crate::common::utils::{setup_convertor_contract_roles, setup_pools};
use nep141_token_convertor_contract::token_receiver::TransferMessage::AddLiquidity;
use crate::common::convertor::ConvertorContract;

mod common;

#[tokio::test]
async fn test_create_pool() {
    let (  whitelist_tokens, token_contracts) = setup_pools().await;
    let (root, owner, creator, user) = setup_convertor_contract_roles().await;
    let convertor_contract: &ConvertorContract = CONVERTOR_CONTRACT.get().await;

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
    convertor_contract.set_deposit_amount_of_pool_creation(&owner, U128(parse_near!("1N")));

    assert!(
        convertor_contract
            .create_pool(
                &creator,
                whitelist_tokens[0].token_id.clone(),
                whitelist_tokens[1].token_id.clone(),
                true,
                1,
                1,
                Option::None
            ).await.is_err(),
        "should failed by attach near not enough"
    );

    convertor_contract
        .create_pool(
            &creator,
            whitelist_tokens[0].token_id.clone(),
            whitelist_tokens[1].token_id.clone(),
            true,
            1,
            1,
            Option::Some(parse_near!("1N")),
        ).await.unwrap();
}

#[tokio::test]
async fn test_deposit_withdraw_delete() {
    let (  whitelist_tokens, token_contracts) = setup_pools().await;
    let (root, owner, creator, user) = setup_convertor_contract_roles().await;
    let convertor_contract = CONVERTOR_CONTRACT.get().await;

    convertor_contract
        .set_deposit_amount_of_pool_creation(&owner, U128::from(1))
        .await.unwrap();
    convertor_contract
        .create_pool(
            &creator,
            whitelist_tokens[0].token_id.clone(),
            whitelist_tokens[1].token_id.clone(),
            true,
            1,
            1,
            Option::Some(1),
        ).await.unwrap();

    let token0 = &token_contracts[0];
    token0
        .mint( creator.id().clone(), U128::from(100))
        .await.unwrap();

    token0
        .ft_transfer_call(
            &creator,
            convertor_contract.contract_id.clone(),
            U128::from(10),
            Option::None,
            json!(AddLiquidity { pool_id: U64(1) }).to_string(),
        ).await.unwrap();

    assert_eq!(
        10,
        convertor_contract.get_pools(0, 1).await.pop().unwrap().in_token_balance.0
    );

    convertor_contract
        .withdraw_token_in_pool(
            &creator,
            U64(1),
            whitelist_tokens[0].token_id.clone(),
            Option::None,
        ).await.unwrap();
    assert_eq!(
        0,
        convertor_contract.get_pools(0, 1).await.pop().unwrap().in_token_balance.0
    );

    convertor_contract
        .withdraw_token_in_pool(
            &creator,
            U64(1),
            whitelist_tokens[1].token_id.clone(),
            Option::None,
        ).await.unwrap();

    assert_eq!(
        0,
        convertor_contract.get_pools(0, 1).await.pop().unwrap().out_token_balance.0
    );

    assert!(convertor_contract.delete_pool(&root, U64(1)).await.is_err(),"should delete failed");
    convertor_contract.delete_pool(&creator, U64(1)).await.unwrap();
    let balance = token0.ft_balance_of(creator.id().clone()).await;
    assert_eq!(100, balance.0);
}
