use crate::common::*;
use crate::constant::{convertor_contract_id, string_to_account};
use crate::contracts::{deploy_test_token_contract, setup_convertor_contract, should_failed};
use crate::convertor::{Convertor, setup_pools};
use itertools::Itertools;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{call, to_yocto, view, ContractAccount, UserAccount};
use nep141_token_convertor_contract::token_receiver::TransferMessage::AddLiquidity;
use test_token::ContractContract as TestTokenContract;

pub mod common;



#[test]
fn test_create_pool() {
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
    convertor.set_pool_create_deposit_amount(&admin, U128(to_yocto("1")));

    assert!(
        !convertor
            .create_pool(
                &creator,
                whitelist_tokens[0].token_id.clone(),
                whitelist_tokens[1].token_id.clone(),
                true,
                1,
                1,
                Option::None
            )
            .is_ok(),
        "should failed by attach near not enough"
    );

    convertor
        .create_pool(
            &creator,
            whitelist_tokens[0].token_id.clone(),
            whitelist_tokens[1].token_id.clone(),
            true,
            1,
            1,
            Option::Some(to_yocto("1")),
        )
        .assert_success();
}

#[test]
fn test_deposit_withdraw_delete() {
    let (root, admin, convertor, creator, user, whitelist_tokens, token_contracts) = setup_pools();

    convertor
        .set_pool_create_deposit_amount(&admin, U128::from(1))
        .assert_success();
    convertor
        .create_pool(
            &creator,
            whitelist_tokens[0].token_id.clone(),
            whitelist_tokens[1].token_id.clone(),
            true,
            1,
            1,
            Option::Some(1),
        )
        .assert_success();

    let token0: &ContractAccount<TestTokenContract> = &token_contracts[0];
    call!(
        root,
        token0.mint(string_to_account("creator"), U128::from(100))
    )
    .assert_success();

    call!(
        creator,
        token0.ft_transfer_call(
            convertor_contract_id(),
            U128::from(10),
            Option::None,
            json!(AddLiquidity { pool_id: 1 }).to_string()
        ),
        deposit = 1
    )
    .assert_success();

    assert_eq!(
        10,
        convertor.get_pools(0, 1).pop().unwrap().in_token_balance.0
    );

    convertor
        .withdraw_token(
            &creator,
            1,
            whitelist_tokens[0].token_id.clone(),
            U128::from(5),
        )
        .assert_success();

    assert_eq!(
        5,
        convertor.get_pools(0, 1).pop().unwrap().in_token_balance.0
    );

    should_failed(&convertor.delete_pool(&root, 1));
    convertor.delete_pool(&creator, 1).assert_success();
    let balance = view!(token0.ft_balance_of(string_to_account("creator"))).unwrap_json::<U128>();
    assert_eq!(100,balance.0);
}