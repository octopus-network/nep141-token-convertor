use near_sdk::json_types::U128;
use workspaces::Account;
use workspaces::prelude::*;
use nep141_token_convertor_contract::FtMetaData;

use crate::common::constant::{CONVERTOR_CONTRACT, USDT_ACCOUNT, USDC_ACCOUNT, USDN_ACCOUNT};
use crate::common::utils::{setup_convertor_contract_roles, setup_pools};
use crate::common::convertor::ConvertorContract;

mod common;

#[tokio::test]
async fn test_whitelist() {
    let (  whitelist_tokens, token_contracts) = setup_pools().await;
    let (root, owner, creator, user) = setup_convertor_contract_roles().await;
    let convertor_contract: &ConvertorContract= CONVERTOR_CONTRACT.get().await;

    let usdt: &Account = USDT_ACCOUNT.get().await;
    let usdc: &Account = USDC_ACCOUNT.get().await;
    let usdn: &Account = USDN_ACCOUNT.get().await;


    let mut tokens = vec![
        FtMetaData {
            token_id: near_sdk::AccountId::new_unchecked(usdt.id().to_string()),
            decimals: 6,
        },
        FtMetaData {
            token_id: near_sdk::AccountId::new_unchecked(usdc.id().to_string()),
            decimals: 6,
        },
        FtMetaData {
            token_id: near_sdk::AccountId::new_unchecked(usdn.id().to_string()),
            decimals: 8,
        },
    ];
    convertor_contract
        .extend_whitelisted_tokens(&owner, tokens.clone()).await.unwrap();
    assert_eq!(
        convertor_contract.get_whitelist().await,
        tokens,
        "extend whitelist not right "
    );

    tokens.pop();
    tokens.pop();
    let remove_token_ids = vec![near_sdk::AccountId::new_unchecked(usdn.id().to_string()),near_sdk::AccountId::new_unchecked(usdc.id().to_string())];
    convertor_contract.remove_whitelisted_tokens(&owner, remove_token_ids.clone()).await.unwrap();
    assert_eq!(convertor_contract.get_whitelist().await, tokens, "remove token not right");
    assert!(
        convertor_contract
            .extend_whitelisted_tokens(&root, tokens.clone()).await
            .is_err(),
        "should failed by owner access check"
    );
    assert!(
        convertor_contract
            .remove_whitelisted_tokens(&root, remove_token_ids.clone()).await
            .is_err(),
        "should failed by owner access check"
    )
}

#[tokio::test]
async fn test_set_pool_create_deposit_amount() {
    let (  whitelist_tokens, token_contracts) = setup_pools().await;
    let (root, owner, creator, user) = setup_convertor_contract_roles().await;
    let convertor_contract: &ConvertorContract= CONVERTOR_CONTRACT.get().await;

    assert!(
        convertor_contract
            .set_deposit_amount_of_pool_creation(&root, U128::from(1)).await
            .is_err(),
        "should failed by owner access check"
    );
    convertor_contract
        .set_deposit_amount_of_pool_creation(&owner, U128::from(1))
        .await.unwrap();
}
