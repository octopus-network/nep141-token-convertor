use anyhow::anyhow;
use itertools::Itertools;
use near_sdk::json_types::U128;
use near_sdk::{AccountId, Balance, serde_json};
use near_sdk::serde_json::json;
use near_units::parse_near;
use workspaces::result::CallExecutionDetails;
use nep141_token_convertor_contract::account::AccountView;
use nep141_token_convertor_contract::constants::T_GAS_FOR_RESOLVE_TRANSFER;
use nep141_token_convertor_contract::conversion_pool::ConversionPool;
use nep141_token_convertor_contract::types::PoolId;
use nep141_token_convertor_contract::FtMetaData;
use workspaces::Account;
use crate::common::constant::{CONVERTOR_CONTRACT, CONVERTOR_CONTRACT_ACCOUNT, CONVERTOR_OWNER_ACCOUNT, CREATOR_ACCOUNT, ROOT, USDC_ACCOUNT, USDN_ACCOUNT, USDT_ACCOUNT, USER_ACCOUNT, WORKER};
use crate::common::nep141::Nep141;

pub struct ConvertorContract {
    pub contract_id: workspaces::AccountId,
}

impl ConvertorContract {
    pub async fn get_account(&self, account_id: AccountId) -> AccountView {
        WORKER.get().await.view(
            &self.contract_id,
            "get_account",
            json!((account_id)).to_string()
                .into_bytes())
            .await.unwrap()
            .json().unwrap()
    }

    pub async fn get_storage_fee_gap_of(&self, account_id: &workspaces::AccountId) -> U128 {
        WORKER.get().await.view(
            &self.contract_id,
            "get_storage_fee_gap_of",
            serde_json::json!({"account_id": account_id}).to_string().into_bytes()
        ).await.unwrap().json().unwrap()
    }

    pub async fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool> {
        WORKER.get().await.view(
            &self.contract_id,
            "get_pools",
            json!((from_index,limit)).to_string().into_bytes())
            .await.unwrap()
            .json()
            .unwrap()
    }

    pub async fn get_whitelist(&self) -> Vec<FtMetaData> {
        WORKER.get().await.view(
            &self.contract_id,
            "get_whitelist",
            json!(()).to_string().into_bytes()
        ).await.unwrap().json().unwrap()
    }

    pub async fn extend_whitelisted_tokens(
        &self,
        signer: &workspaces::Account,
        tokens: Vec<FtMetaData>,
    ) -> anyhow::Result<CallExecutionDetails> {

        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "extend_whitelisted_tokens")
            .args_json(json!({"tokens": tokens}))?
            .transact()
            .await
    }

    pub async fn remove_whitelisted_tokens(
        &self,
        signer: &Account,
        tokens: Vec<near_sdk::AccountId>,
    ) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "remove_whitelisted_tokens")
            .args_json(json!({"tokens": tokens}))?
            .transact()
            .await
    }

    pub async fn set_deposit_amount_of_pool_creation(
        &self,
        signer: &Account,
        amount: U128,
    ) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "set_deposit_amount_of_pool_creation")
            .args_json(json!({"amount": amount}))?
            .transact()
            .await
    }

    pub async fn create_pool(
        &self,
        signer: &Account,
        token_from: AccountId,
        token_to: AccountId,
        is_reversible: bool,
        in_token_rate: u32,
        out_token_rate: u32,
        attach: Option<Balance>,
    ) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "create_pool")
            .deposit(attach.unwrap_or(0))
            .args_json(json!((
                token_from,
                token_to,
                is_reversible,
                in_token_rate,
                out_token_rate,
            )))?
            .transact()
            .await
    }

    pub async fn register_account(&self, signer: &Account) -> anyhow::Result<CallExecutionDetails> {
        let storage_fee = self.get_storage_fee_gap_of(signer.id()).await.0;
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "storage_deposit")
            .deposit(storage_fee)
            .args_json(json!({}))?
            .transact()
            .await
    }

    pub async fn withdraw_token_in_pool(
        &self,
        signer: &Account,
        pool_id: PoolId,
        token_id: AccountId,
        amount: Option<U128>,
    ) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "withdraw_token_in_pool")
            .deposit(1)
            .max_gas()
            .args_json(json!({
                "pool_id": pool_id,
                "token_id": token_id,
                "amount": amount
            }))?
            .transact()
            .await
    }

    pub async fn delete_pool(&self, signer: &Account, pool_id: PoolId) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "delete_pool")
            .deposit(1)
            .max_gas()
            .args_json(json!({"pool_id": pool_id}))?
            .transact()
            .await
    }
}
