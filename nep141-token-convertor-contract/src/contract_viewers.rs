use crate::account::AccountView;
use crate::contract_interfaces::ConvertorViewer;
use crate::conversion_pool::ConversionPool;
use crate::*;

#[near_bindgen]
impl ConvertorViewer for TokenConvertor {
    fn get_whitelist(&self) -> Vec<FtMetaData> {
        self.whitelisted_tokens.values().collect_vec()
    }

    fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool> {
        self.pools
            .iter()
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|e| e.1.into_current())
            .collect_vec()
    }

    fn get_pools_by_creator(&self, account_id: AccountId) -> Vec<ConversionPool> {
        self.pools
            .iter()
            .map(|e| e.1.into_current())
            .filter(|e| e.creator == account_id)
            .collect_vec()
    }

    /// storage fee need deposit = storage_balance_bounds.min - account.near_amount_for_storage
    /// if account.near_amount_for_storage > storage_balance_bounds.min,it should return 0
    fn get_storage_fee_gap_of(&self, account_id: AccountId) -> U128 {
        let near_amount_for_storage = self
            .internal_get_account(&account_id)
            .map(|e| e.near_amount_for_storage)
            .unwrap_or(0);
        let min_bound = self.internal_get_storage_balance_min_bound(&account_id);
        return if near_amount_for_storage >= min_bound {
            U128(0)
        } else {
            U128(min_bound - near_amount_for_storage)
        };
    }

    fn get_account(&self, account_id: AccountId) -> AccountView {
        self.internal_get_account(&account_id)
            .expect("no such account")
            .into()
    }

    fn is_contract_paused(&self) -> bool {
        self.contract_is_paused
    }

    fn get_deposit_amount_of_pool_creation(&self) -> U128 {
        U128(self.create_pool_deposit)
    }
}
