use crate::common::convertor::ConvertorContract;
use crate::common::nep141::Nep141;
use futures;
use near_sdk::serde_json::json;
use near_units::parse_near;
use nep141_token_convertor_contract::FtMetaData;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};

pub const CONVERTOR_WASM_BYTES: &[u8] =
    include_bytes!("../../../res/nep141_token_convertor_contract.wasm");
pub const TEST_TOKEN_WASM_BYTES: &[u8] = include_bytes!("../../../res/test_token.wasm");

pub async fn register_account(worker: &Worker<Sandbox>, root: &Account, name: &str) -> Account {
    let tt = root
        .create_subaccount(worker, name)
        .initial_balance(parse_near!("100 N"))
        .transact()
        .await;
    tt.unwrap().into_result().unwrap()
}

pub async fn setup_pools() -> (
    Worker<Sandbox>,
    Vec<FtMetaData>,
    Vec<Nep141>,
    ConvertorContract,
    Account,
    Account,
    Account,
    Account,
) {
    let worker = workspaces::sandbox().await.unwrap();
    let root = worker.root_account();

    let owner = register_account(&worker, &root, "owner").await;
    let creator = register_account(&worker, &root, "creator").await;
    let user = register_account(&worker, &root, "user").await;
    let convertor = register_account(&worker, &root, "convertor").await;

    let accounts_to_register = vec![
        convertor.id().to_string(),
        creator.id().to_string(),
        user.id().to_string(),
    ];

    let usdt = register_account(&worker, &root, "usdt").await;
    let usdc = register_account(&worker, &root, "usdc").await;
    let usdn = register_account(&worker, &root, "usdn").await;

    deploy_test_token_contract(&worker, &usdt, accounts_to_register.clone()).await;
    deploy_test_token_contract(&worker, &usdc, accounts_to_register.clone()).await;
    deploy_test_token_contract(&worker, &usdn, accounts_to_register.clone()).await;

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

    let convertor_contract = ConvertorContract::deploy(&worker, &convertor, &owner).await;
    convertor_contract
        .extend_whitelisted_tokens(&worker, &owner, whitelist_tokens.clone())
        .await
        .unwrap();

    convertor_contract
        .register_account(&worker, &user)
        .await
        .unwrap();
    convertor_contract
        .register_account(&worker, &creator)
        .await
        .unwrap();

    let token_contracts = vec![
        Nep141 {
            contract_id: usdt.id().clone(),
            account: usdt,
        },
        Nep141 {
            contract_id: usdc.id().clone(),
            account: usdc,
        },
        Nep141 {
            contract_id: usdn.id().clone(),
            account: usdn,
        },
    ];
    return (
        worker,
        whitelist_tokens,
        token_contracts,
        convertor_contract,
        root,
        owner,
        creator,
        user,
    );
}

pub async fn deploy_test_token_contract(
    worker: &Worker<Sandbox>,
    signer_account: &workspaces::Account,
    accounts_to_register: Vec<String>,
) {
    signer_account
        .deploy(worker, TEST_TOKEN_WASM_BYTES)
        .await
        .unwrap();
    signer_account
        .call(worker, signer_account.id(), "new")
        .args_json(())
        .unwrap()
        .transact()
        .await
        .unwrap();

    let mut i = 0;
    loop {
        let account_id = accounts_to_register[i].clone();
        let result = signer_account
            .call(worker, signer_account.id(), "storage_deposit")
            .deposit(parse_near!("0.00125 N"))
            .args_json(json!({ "account_id": account_id }))
            .unwrap()
            .transact()
            .await
            .unwrap();
        i += 1;
        if i == accounts_to_register.len() {
            break;
        }
    }
}
