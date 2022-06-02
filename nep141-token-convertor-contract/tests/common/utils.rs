use std::future::Future;
use futures;
use near_sdk::serde_json::json;
use near_units::parse_near;
use workspaces::Account;
use workspaces::result::CallExecutionDetails;
use nep141_token_convertor_contract::FtMetaData;
use crate::common::nep141::Nep141;
use crate::common::constant::{CONVERTOR_OWNER_ACCOUNT, CONVERTOR_CONTRACT, CREATOR_ACCOUNT, ROOT, TEST_TOKEN_WASM_BYTES, USDC_ACCOUNT, USDN_ACCOUNT, USDT_ACCOUNT, USER_ACCOUNT, WORKER, INIT_TEST_TOKENS};

async fn join_parallel<T: Send + 'static>(
    futs: impl IntoIterator<Item = impl Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let tasks: Vec<_> = futs.into_iter().map(tokio::spawn).collect();
    // unwrap the Result because it is introduced by tokio::spawn()
    // and isn't something our caller can handle
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

pub async fn setup_pools() -> (
    Vec<FtMetaData>,
    Vec<Nep141>,
) {
    let (root, owner, creator, user) = setup_convertor_contract_roles().await;
    // let creator = root.create_user(string_to_account("creator"), to_yocto("100"));
    // let user = root.create_user(string_to_account("user"), to_yocto("100"));

    INIT_TEST_TOKENS.get().await;

    let convertor_contract = CONVERTOR_CONTRACT.get().await;

    convertor_contract.register_account(&user).await.unwrap();
    convertor_contract.register_account(&creator).await.unwrap();

    let usdt = USDT_ACCOUNT.get().await;
    let usdc = USDC_ACCOUNT.get().await;
    let usdn = USDN_ACCOUNT.get().await;

    let whitelist_tokens = vec![
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
            decimals: 6,
        },
    ];
    convertor_contract.extend_whitelisted_tokens(&owner, whitelist_tokens.clone()).await.unwrap();

    let token_contracts = vec![
        Nep141{ account: usdt, contract_id: usdt.id().clone() },
        Nep141{ account: usdc, contract_id: usdc.id().clone() },
        Nep141{ account: usdn, contract_id: usdn.id().clone() },
    ];

    return (
        whitelist_tokens,
        token_contracts,
    );
}



pub async fn deploy_test_token_contract(
    signer_account: &'static workspaces::Account,
    accounts_to_register: Vec<String>,
) {
    let worker = WORKER.get().await;
    signer_account.deploy(worker,TEST_TOKEN_WASM_BYTES).await.unwrap();
    signer_account.call(worker,signer_account.id(),"new")
        .args_json(())
        .unwrap().transact().await.unwrap();
    join_parallel(accounts_to_register.into_iter().map(|account_id| async move {
        signer_account.call(worker,signer_account.id(),"storage_deposit")
            .deposit(parse_near!("0.00125 N"))
            .args_json(json!({"account_id": account_id})).unwrap()
            .transact().await.unwrap();
    })).await;
}

pub async fn setup_convertor_contract_roles()
    -> (&'static Account, &'static Account, &'static Account, &'static Account) {
    let root = ROOT.get().await;
    let owner = CONVERTOR_OWNER_ACCOUNT.get().await;
    let creator = CREATOR_ACCOUNT.get().await;
    let user = USER_ACCOUNT.get().await;
    return (root,owner,creator,user)
}

trait Print {
    fn print(&self);
}

impl Print for CallExecutionDetails {
    fn print(&self) {
        println!("{:?}", self)
    }
}