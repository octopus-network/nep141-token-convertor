use async_once::AsyncOnce;
use lazy_static::lazy_static;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_units::{parse_gas, parse_near};
// use serde::Deserialize;
// use serde_json::json;

use workspaces::network::Sandbox;
use workspaces::prelude::*;
use workspaces::{Account, AccountId, Contract, Worker};
// mod common;

const CONVERTOR_WASM_BYTES: &[u8] = include_bytes!("../../res/nep141_token_convertor_contract.wasm");
const TEST_TOKEN_WASM_BYTES: &[u8] = include_bytes!("../../res/test_token.wasm");


lazy_static! {
    static ref WORKER: AsyncOnce<Worker<Sandbox>> =AsyncOnce::new(async {
       workspaces::sandbox().await.unwrap()
    });
    static ref USDT: AccountId= string_to_account("xsb.test");
}

// static  WORKER: Worker<Sandbox> = workspaces::sandbox().await?;


pub async fn _deploy_convertor_contract(
    owner: &AccountId,
    deposit_near_amount: U128,
) -> anyhow::Result<Contract> {
    // let  WORKER: Worker<Sandbox> = workspaces::sandbox().await?;
    let worker = WORKER.get().await;

    let contract = worker.dev_deploy(CONVERTOR_WASM_BYTES).await?;
    // (owner: owner, create_pool_deposit: parse_near!("1 N"))
    contract
        .call(worker, "new")
        .args_json(
         (owner.clone(), parse_near!("1 N").to_string())
        )?
        .transact()
        .await?;
    Ok(contract)
}


pub fn string_to_account(name: &str) -> AccountId {

    AccountId::try_from(name.to_string()).unwrap()
}

#[tokio::test]
async fn test()->anyhow::Result<()> {
    let worker = WORKER.get().await;
    let root = worker.root_account();
    println!("{:?}",root.id());

    let contract = _deploy_convertor_contract(
        &USDT,
        U128(1)
    ).await?;
    println!("{:?}", contract.as_account().id());
    println!("{}", parse_near!("7yN"));
    Ok(())
}