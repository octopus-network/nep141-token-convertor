use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::result::CallExecutionDetails;
use workspaces::{Account, AccountId, Worker};

pub struct Nep141 {
    pub account: workspaces::Account,
    pub contract_id: workspaces::AccountId,
}

impl Nep141 {
    pub async fn ft_transfer_call(
        &self,
        worker: &Worker<Sandbox>,
        signer: &workspaces::Account,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(worker, &self.contract_id, "ft_transfer_call")
            .deposit(1)
            .max_gas()
            .args_json(json!((receiver_id, amount, memo, msg)))?
            .transact()
            .await
    }

    pub async fn mint(
        &self,
        worker: &Worker<Sandbox>,
        account_id: workspaces::AccountId,
        amount: U128,
    ) -> anyhow::Result<CallExecutionDetails> {
        // let worker = WORKER.get().await;
        self.account
            .call(worker, &self.contract_id, "mint")
            .args_json(json!((account_id, amount)))?
            .transact()
            .await
    }

    pub async fn ft_balance_of(&self, worker: &Worker<Sandbox>, account_id: AccountId) -> U128 {
        let result = worker
            .view(
                &self.contract_id,
                "ft_balance_of",
                json!({ "account_id": account_id }).to_string().into_bytes(),
            )
            .await;
        let a = result.unwrap();
        a.json().unwrap()
    }

    pub async fn storage_deposit(
        &self,
        worker: &Worker<Sandbox>,
        signer: &workspaces::Account,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
        amount: u128,
    ) -> CallExecutionDetails {
        let result = signer
            .call(worker, &self.contract_id, "storage_deposit")
            .deposit(amount)
            .args_json(json!((account_id, registration_only)))
            .unwrap()
            .transact()
            .await
            .unwrap();
        result
    }
}
