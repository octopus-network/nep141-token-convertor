extern crate core;

pub mod account;
pub mod constants;
pub mod contract_interfaces;
pub mod contract_viewers;
pub mod conversion_pool;
pub mod external_trait;
pub mod owner;
pub mod storage_impl;
pub mod token_receiver;
pub mod types;

use crate::account::VAccount;
use crate::conversion_pool::VPool;
pub use crate::types::FtMetaData;
use itertools::Itertools;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, log, near_bindgen, serde_json, AccountId, Balance, BorshStorageKey, Gas,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};
use types::PoolId;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenConvertor {
    pub owner: AccountId,
    pub accounts: LookupMap<AccountId, VAccount>,
    pub pools: UnorderedMap<PoolId, VPool>,
    pub whitelisted_tokens: UnorderedMap<AccountId, FtMetaData>,
    // request deposit some near when creating pool.owner can change it.
    pub create_pool_deposit: Balance,
    // auto increase id.
    pub pool_id: u64,
    pub contract_is_paused: bool,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Pools,
    Accounts,
    WhitelistedTokens,
}

#[near_bindgen]
impl TokenConvertor {
    #[init]
    pub fn new(owner: AccountId, create_pool_deposit: U128) -> Self {
        Self {
            owner,
            accounts: LookupMap::new(StorageKey::Accounts),
            pools: UnorderedMap::new(StorageKey::Pools),
            whitelisted_tokens: UnorderedMap::new(StorageKey::WhitelistedTokens),
            create_pool_deposit: create_pool_deposit.0,
            pool_id: 0,
            contract_is_paused: false,
        }
    }

    pub(crate) fn assert_token_in_whitelist(&self, token: &AccountId) {
        assert!(
            self.whitelisted_tokens.get(token).is_some(),
            "Token '{}' is not in the whitelist.",
            token
        );
    }

    pub(crate) fn assert_create_pool_deposit_amount(&self) {
        assert_eq!(
            env::attached_deposit(),
            self.create_pool_deposit,
            "Creating a pool requires a deposit of '{}' yocto NEAR.",
            self.create_pool_deposit
        );
    }

    pub(crate) fn assert_contract_is_not_paused(&self) {
        assert!(!self.contract_is_paused, "contract is paused.")
    }

    pub(crate) fn assert_storage_balance_bound_min(&self, account_id: &AccountId) {
        let account = self
            .internal_get_account(account_id)
            .expect(format!("The account '{}' is not registered.", account_id).as_str());

        let min_bound = self.internal_get_storage_balance_min_bound(&account_id);
        assert!(
            account.near_amount_for_storage >= min_bound,
            "Need to deposit at least '{}' yocto NEAR as storage fee.",
            min_bound - account.near_amount_for_storage
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub mod test {
    use crate::TokenConvertor;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};
    use std::convert::TryFrom;

    pub fn string_to_account(name: &str) -> AccountId {
        AccountId::try_from(name.to_string()).unwrap()
    }

    pub fn setup_contract() -> (VMContextBuilder, TokenConvertor, AccountId) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        testing_env!(context.attached_deposit(1).build());
        testing_env!(context.block_timestamp(1638790720000).build());
        let owner = AccountId::try_from("owner.near".to_string()).unwrap();
        let contract = TokenConvertor::new(owner.clone(), U128(0));
        (context, contract, owner.clone())
    }
}
