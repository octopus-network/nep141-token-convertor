use near_sdk::json_types::U128;
use workspaces::prelude::*;
use nep141_token_convertor_contract::FtMetaData;

use crate::common::utils::setup_pools;

mod common;

#[tokio::test]
async fn test_whitelist() {

    let (  worker, whitelist_tokens, token_contracts, convertor_contract, root, owner, creator, user ) = setup_pools().await;

    let usdt_id = whitelist_tokens[0].token_id.clone();
    let usdc_id = whitelist_tokens[1].token_id.clone();
    let usdn_id = whitelist_tokens[2].token_id.clone();

    let mut tokens: Vec<FtMetaData> = whitelist_tokens.iter()
        .map(|e|FtMetaData{
            token_id: e.token_id.clone(),
            decimals: 6
        }).collect();
    tokens[2].decimals=8;

    let mut tokens = vec![
        FtMetaData {
            token_id: usdt_id.clone(),
            decimals: 6,
        },
        FtMetaData {
            token_id: usdc_id.clone(),
            decimals: 6,
        },
        FtMetaData {
            token_id: usdn_id.clone(),
            decimals: 8,
        },
    ];
    convertor_contract
        .extend_whitelisted_tokens(&worker, &owner, tokens.clone()).await.unwrap();
    assert_eq!(
        convertor_contract.get_whitelist(&worker).await,
        tokens,
        "extend whitelist not right "
    );

    tokens.pop();
    tokens.pop();
    let remove_token_ids = vec![usdn_id.clone(),usdc_id.clone()];
    convertor_contract.remove_whitelisted_tokens(&worker, &owner, remove_token_ids.clone()).await.unwrap();
    assert_eq!(convertor_contract.get_whitelist(&worker).await, tokens, "remove token not right");
    assert!(
        convertor_contract
            .extend_whitelisted_tokens(&worker, &root, tokens.clone()).await
            .is_err(),
        "should failed by owner access check"
    );
    assert!(
        convertor_contract
            .remove_whitelisted_tokens(&worker,&root, remove_token_ids.clone()).await
            .is_err(),
        "should failed by owner access check"
    )
}

#[tokio::test]
async fn test_set_pool_create_deposit_amount() {
    let (  worker, whitelist_tokens, token_contracts, convertor_contract, root, owner, creator, user ) = setup_pools().await;

    assert!(
        convertor_contract
            .set_deposit_amount_of_pool_creation(&worker, &root, U128::from(1)).await
            .is_err(),
        "should failed by owner access check"
    );
    convertor_contract
        .set_deposit_amount_of_pool_creation(&worker, &owner, U128::from(1))
        .await.unwrap();
}
