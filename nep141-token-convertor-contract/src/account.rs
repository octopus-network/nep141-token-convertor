use std::collections::HashMap;

use near_sdk::StorageUsage;

use crate::*;

const U128_STORAGE: StorageUsage = 16;
const U64_STORAGE: StorageUsage = 8;
const U32_STORAGE: StorageUsage = 4;
/// max length of account id is 64 bytes. We charge per byte.
const ACC_ID_STORAGE: StorageUsage = 64;
/// As a key, 4 bytes length would be added to the head
const ACC_ID_AS_KEY_STORAGE: StorageUsage = ACC_ID_STORAGE + 4;
/// As a near_sdk::collection key, 1 byte for prefiex
const ACC_ID_AS_CLT_KEY_STORAGE: StorageUsage = ACC_ID_AS_KEY_STORAGE + 1;

// ACC_ID: the Contract accounts map key length
// + VAccount enum: 1 byte
// + U128_STORAGE: storage_near_amount storage
// + U32_STORAGE: tokens HashMap length
pub const INIT_ACCOUNT_STORAGE: StorageUsage =
    ACC_ID_AS_CLT_KEY_STORAGE + 1 + U32_STORAGE + U128_STORAGE;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VAccount {
    Current(Account),
}

impl VAccount {
    /// Upgrades from other versions to the currently used version.
    #[allow(unused)]
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

    pub fn withdraw_tokens(&mut self, token_id: &AccountId, amount: Balance) {
        let balance = *self
            .tokens
            .get(token_id)
            .expect("Fail to withdraw nonexistent token.");
        assert!(
            balance >= amount,
            "Fail to withdraw ft {{contract_id: {}, amount: {}}}, account balance is {}",
            token_id,
            amount,
            balance
        );
        self.tokens.insert(token_id.clone(), balance - amount);
    }

    pub fn storage_usage(&self) -> u64 {
        INIT_ACCOUNT_STORAGE + self.tokens.len() as u64 * (ACC_ID_AS_KEY_STORAGE + U128_STORAGE)
    }

    pub fn storage_cost(&self) -> Balance {
        self.storage_usage() as u128 * env::storage_byte_cost()
    }

    pub fn storage_debt(&self) -> Balance {
        if self.near_amount_for_storage < self.storage_cost() {
            self.storage_cost() - self.near_amount_for_storage
        } else {
            0
        }
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

    pub(crate) fn internal_use_account<F, R>(
        &mut self,
        account_id: &AccountId,
        check_deposit: bool,
        mut f: F,
    ) -> R
    where
        F: FnMut(&mut Account) -> R,
    {
        let mut account = self
            .internal_get_account(account_id)
            .expect("No such account");
        let r = f(&mut account);
        self.internal_save_account(account_id, account, check_deposit);
        r
    }

    pub(crate) fn internal_save_account(
        &mut self,
        account_id: &AccountId,
        account: Account,
        check_deposit: bool,
    ) {
        if check_deposit {
            assert_eq!(
                account.storage_debt(),
                0,
                "Need pay {} yoctoNear  for storage debt.",
                account.storage_debt()
            );
        }
        self.accounts.insert(account_id, &account.into());
    }
}
