extern crate core;

pub mod account;
pub mod admin;
pub mod constants;
pub mod contract_interfaces;
pub mod contract_viewers;
pub mod conversion_pool;
pub mod external_trait;
pub mod storage_impl;
pub mod token_receiver;
pub mod types;

use crate::account::VAccount;
use crate::conversion_pool::VPool;
pub use crate::types::{FtMetaData, TokenDirectionKey};
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
    pub admin: AccountId,
    pub accounts: LookupMap<AccountId, VAccount>,
    pub pools: UnorderedMap<PoolId, VPool>,
    pub whitelisted_tokens: UnorderedMap<AccountId, FtMetaData>,
    // request deposit some near when creating pool.admin can change it.
    pub create_pool_deposit: Balance,
    // auto increase id.
    pub pool_id: u32,
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
    pub fn new(admin: AccountId) -> Self {
        Self {
            admin,
            accounts: LookupMap::new(StorageKey::Accounts),
            pools: UnorderedMap::new(StorageKey::Pools),
            whitelisted_tokens: UnorderedMap::new(StorageKey::WhitelistedTokens),
            create_pool_deposit: 0,
            pool_id: 0,
            contract_is_paused: false,
        }
    }

    pub(crate) fn assert_token_in_whitelist(&self, token: &AccountId) {
        assert!(
            self.whitelisted_tokens.get(token).is_some(),
            "token {} is not in whitelist",
            token
        );
    }

    pub(crate) fn assert_create_pool_deposit_amount(&self) {
        assert_eq!(
            env::attached_deposit(),
            self.create_pool_deposit,
            "Create pool must deposit {} yocoto near",
            self.create_pool_deposit
        );
    }

    pub(crate) fn assert_contract_is_not_paused(&self) {
        assert!(!self.contract_is_paused, "contract is paused")
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub mod test {
    use crate::{Account, TokenConvertor};
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId, VMContext};
    use std::convert::TryFrom;

    pub const USDT: AccountId = AccountId::try_from("usdt.near".to_string()).unwrap();
    pub const USDC: AccountId = AccountId::try_from("usdc.near".to_string()).unwrap();

    pub fn setup_contract() -> (VMContextBuilder, TokenConvertor, AccountId) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        testing_env!(context.attached_deposit(ONE_YOCTO).build());
        testing_env!(context.block_timestamp(1638790720000).build());
        let whitelist_admin = AccountId::try_from("whitelist_admin.near".to_string()).unwrap();
        let contract = TokenConvertor::new(whitelist_admin.clone());
        (context, contract, whitelist_admin.clone())
    }
}
