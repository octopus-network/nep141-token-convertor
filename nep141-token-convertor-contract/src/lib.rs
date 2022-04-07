extern crate core;

mod types;
mod contract_interfacs;
mod conversion_pool;
mod token_receiver;
mod admin;
mod account;
mod external_trait;
mod storage_impl;
mod constants;
mod contract_viewers;

use itertools::Itertools;
use near_sdk::{assert_self,BorshStorageKey, env, ext_contract, log, near_bindgen, serde_json, AccountId, Balance, Gas, PanicOnDefault, PromiseOrValue, PromiseResult, Timestamp, StorageUsage, Promise};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use types::PoolId;
use crate::account::{Account, VAccount};
use crate::conversion_pool::Pool;
use crate::types::{TokenDirectionKey, FtMetaData};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize, };

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenConvertor {
    // The owner account Id
    admin: AccountId,
    accounts: LookupMap<AccountId, VAccount>,
    pools: Vector<Pool>,
    whitelisted_tokens: UnorderedMap<AccountId, FtMetaData>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Pools,
    Accounts,
    WhitelistedTokens
}

#[near_bindgen]
impl TokenConvertor {
    #[init]
    pub fn new(white_list_admin: AccountId) ->Self{
        Self {
            admin: white_list_admin,
            accounts: LookupMap::new(StorageKey::Accounts),
            pools: Vector::new(StorageKey::Pools),
            whitelisted_tokens: UnorderedMap::new(StorageKey::WhitelistedTokens)
        }
    }

    /// Check how much storage taken costs and refund the left over back.
    #[private]
    pub(crate) fn internal_storage_deposit(&mut self, prev_storage: StorageUsage) {
        let storage_cost = env::storage_usage()
            .checked_sub(prev_storage)
            .unwrap_or_default() as Balance
            * env::storage_byte_cost();
        // println!("need: {}, attached: {}", storage_cost, env::attached_deposit());
        let refund = env::attached_deposit()
            .checked_sub(storage_cost)
            .expect("ERR_STORAGE_DEPOSIT");
        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }

    #[private]
    pub(crate) fn asset_token_in_whitelist(&self, token: &AccountId) {
        assert!(self.whitelisted_tokens.get(token).is_some(),"token {} is not in whitelist", token);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub mod test {
    use near_sdk::{AccountId, testing_env, VMContext};
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use crate::{Account, TokenConvertor};

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