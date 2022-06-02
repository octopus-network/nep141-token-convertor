use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use workspaces::{AccountId, Account};
use workspaces::result::CallExecutionDetails;
use crate::common::constant::WORKER;

pub struct Nep141 {
    pub account: &'static workspaces::Account,
    pub contract_id: workspaces::AccountId,
}

impl Nep141 {
    pub async fn ft_transfer_call(
        &self,
        signer: &workspaces::Account,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        signer
            .call(worker, &self.contract_id, "ft_transfer_call")
            .deposit(1)
            .max_gas()
            .args_json(json!((receiver_id,amount,memo,msg)))?
            .transact()
            .await
    }

    pub async fn mint(
        &self,
        account_id: workspaces::AccountId,
        amount: U128,
    ) -> anyhow::Result<CallExecutionDetails> {
        let worker = WORKER.get().await;
        self.account
            .call(worker, &self.contract_id, "mint")
            .args_json(json!((account_id, amount)))?
            .transact()
            .await
    }

    pub async fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        let result = WORKER.get().await.view(
            &self.contract_id,
            "ft_balance_of",
            json!({"account_id": account_id}).to_string().into_bytes())
            .await;
           let a =  result.unwrap();
            a.json().unwrap()
    }

    pub async fn storage_deposit(
        &self,
        signer: &workspaces::Account,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
        amount: u128
    )-> CallExecutionDetails {
        let worker = WORKER.get().await;
        let result = signer
            .call(worker,&self.contract_id, "storage_deposit")
            .deposit(amount)
            .args_json(json!((account_id,registration_only))).unwrap()
            .transact().await.unwrap();
        result

    }
}
