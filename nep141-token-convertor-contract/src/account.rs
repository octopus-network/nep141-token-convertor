use std::collections::HashMap;

use near_sdk::assert_one_yocto;

use crate::constants::*;
use crate::contract_interfaces::AccountAction;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VAccount {
    Current(Account),
}

impl VAccount {
    /// Upgrades from other versions to the currently used version.
    #[allow(unused_variables)]
    pub fn into_current(self, account_id: &AccountId) -> Account {
        match self {
            VAccount::Current(account) => account,
        }
    }
}

impl From<Account> for VAccount {
    fn from(account: Account) -> Self {
        VAccount::Current(account)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub near_amount_for_storage: Balance,
    // only record token in whitelist,so HashMap is ok.
    pub tokens: HashMap<AccountId, Balance>,
}

impl Account {
    pub fn new() -> Self {
        Account {
            near_amount_for_storage: 0,
            tokens: HashMap::new(),
        }
    }

    pub fn deposit_token(&mut self, token_id: &AccountId, amount: Balance) {
        self.tokens.insert(
            token_id.clone(),
            amount + self.tokens.get(token_id).unwrap_or(&0),
        );
    }

    pub fn withdraw_all_token(&mut self, token_id: &AccountId) -> Balance {
        let balance = *self
            .tokens
            .get(token_id)
            .expect("Fail to withdraw nonexistent token.");
        self.tokens.remove(token_id);
        return balance;
    }

    pub fn storage_usage(&self) -> u64 {
        INIT_ACCOUNT_STORAGE + self.tokens.len() as u64 * (ACC_ID_AS_KEY_STORAGE + U128_STORAGE)
    }

    pub fn storage_cost(&self) -> Balance {
        self.storage_usage() as u128 * env::storage_byte_cost()
    }

    pub fn storage_available_balance(&self) -> Balance {
        if self.near_amount_for_storage > self.storage_cost() {
            self.near_amount_for_storage - self.storage_cost()
        } else {
            0
        }
    }
}

#[near_bindgen]
impl TokenConvertor {
    pub(crate) fn internal_get_account(&self, account_id: &AccountId) -> Option<Account> {
        return self
            .accounts
            .get(account_id)
            .map(|account| account.into_current(account_id));
    }

    pub(crate) fn internal_use_account<F, R>(&mut self, account_id: &AccountId, mut f: F) -> R
    where
        F: FnMut(&mut Account) -> R,
    {
        let mut account = self
            .internal_get_account(account_id)
            .expect("No such account");
        let r = f(&mut account);
        self.internal_save_account(account_id, account);
        r
    }

    pub(crate) fn internal_save_account(&mut self, account_id: &AccountId, account: Account) {
        assert!(
            account.storage_cost() <= account.near_amount_for_storage,
            "Need pay {} yoctoNear for storage.",
            account.storage_cost() - account.near_amount_for_storage
        );
        self.accounts.insert(account_id, &account.into());
    }
}

#[near_bindgen]
impl AccountAction for TokenConvertor {
    #[payable]
    fn withdraw_token_in_account(&mut self, token_id: AccountId) {
        self.assert_contract_is_not_paused();
        assert_one_yocto();
        let balance: u128 = self.internal_use_account(&env::predecessor_account_id(), |account| {
            return account.withdraw_all_token(&token_id);
        });
        if balance > 0 {
            self.internal_send_tokens(&env::predecessor_account_id(), &token_id, balance);
        }
    }
}

/// a struct for serialize Account
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountView {
    pub near_amount_for_storage: U128,
    pub tokens: HashMap<AccountId, U128>,
}

impl From<Account> for AccountView {
    fn from(account: Account) -> Self {
        Self {
            near_amount_for_storage: U128::from(account.near_amount_for_storage),
            tokens: account
                .tokens
                .iter()
                .map(|(k, v)| (k.clone(), U128::from(*v)))
                .collect(),
        }
    }
}
