use lazy_static::lazy_static;
use async_once::AsyncOnce;
use near_units::parse_near;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};
use crate::common::convertor::ConvertorContract;
use crate::common::utils::deploy_test_token_contract;

pub const CONVERTOR_WASM_BYTES: &[u8] = include_bytes!("../../../res/nep141_token_convertor_contract.wasm");
pub const TEST_TOKEN_WASM_BYTES: &[u8] = include_bytes!("../../../res/test_token.wasm");

pub async fn name_to_account(name: &str) -> Account {
    let worker = WORKER.get().await;
    let root = ROOT.get().await;
    root.create_subaccount(worker, name)
        .initial_balance(parse_near!("10 N"))
        .transact().await.unwrap().into_result().unwrap()
}

lazy_static! {
    pub static ref WORKER: AsyncOnce<Worker<Sandbox>> = AsyncOnce::new(async {
       workspaces::sandbox().await.unwrap()
    });
    pub static ref ROOT: AsyncOnce<Account> = AsyncOnce::new(async {
        WORKER.get().await.root_account()
    });
    pub static ref CONVERTOR_OWNER_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        name_to_account("owner").await
    });
    pub static ref CONVERTOR_CONTRACT_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        let worker = WORKER.get().await;
        let convertor = name_to_account("convertor").await;
        let owner = CONVERTOR_OWNER_ACCOUNT.get().await;
        convertor.deploy(worker, CONVERTOR_WASM_BYTES).await.unwrap();
        convertor
        .call(worker, convertor.id(), "new")
        .args_json((owner.id().clone(),parse_near!("1 N").to_string())).unwrap()
        .transact().await.unwrap();
        convertor
    });
    pub static ref CREATOR_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        name_to_account("creator").await
    });

    pub static ref USER_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        name_to_account("user").await
    });
    pub static ref ACCOUNTS_TO_REGISTER: AsyncOnce<Vec<String>> = AsyncOnce::new(async {
        let convertor_contract = CONVERTOR_CONTRACT.get().await;
        let creator = CREATOR_ACCOUNT.get().await;
        let user = USER_ACCOUNT.get().await;
        vec![
            convertor_contract.contract_id.to_string(),
            creator.id().to_string(),
            user.id().to_string()
        ]
    });
    pub static ref USDT_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        name_to_account("usdt").await
    });

    pub static ref USDC_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        name_to_account("usdc").await
    });

    pub static ref USDN_ACCOUNT: AsyncOnce<Account> = AsyncOnce::new(async {
        name_to_account("usdn").await
    });

    pub static ref CONVERTOR_CONTRACT: AsyncOnce<ConvertorContract> = AsyncOnce::new(async {
        let convertor_account = CONVERTOR_CONTRACT_ACCOUNT.get().await;
        ConvertorContract {contract_id: convertor_account.id().clone()}
    });

    pub static ref INIT_TEST_TOKENS: AsyncOnce<()> = AsyncOnce::new(async {
        let accounts_to_register = ACCOUNTS_TO_REGISTER.get().await;
        let usdt_account = USDT_ACCOUNT.get().await;
        let usdc_account = USDC_ACCOUNT.get().await;
        let usdn_account = USDN_ACCOUNT.get().await;
        deploy_test_token_contract(&usdt_account, accounts_to_register.clone()).await;
        deploy_test_token_contract(&usdc_account, accounts_to_register.clone()).await;
        deploy_test_token_contract(&usdn_account, accounts_to_register.clone()).await;
    });
}

