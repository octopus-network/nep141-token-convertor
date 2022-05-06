use crate::account::AccountView;
use crate::conversion_pool::ConversionPool;
use crate::{FtMetaData, PoolId};
use near_sdk::json_types::U128;
use near_sdk::AccountId;

pub trait ConvertorViewer {
    fn get_whitelist(&self) -> Vec<FtMetaData>;

    fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool>;

    fn get_pools_by_creator(&self, account_id: AccountId) -> Vec<ConversionPool>;

    /// storage fee need deposit = storage_balance_bounds.min - account.near_amount_for_storage
    /// if account.near_amount_for_storage > storage_balance_bounds.min, it should return 0
    fn get_storage_fee_gap_of(&self, account_id: AccountId) -> U128;

    fn get_account(&self, account_id: AccountId) -> AccountView;

    fn is_contract_paused(&self) -> bool;
}

pub trait PoolCreatorAction {
    fn create_pool(
        &mut self,
        token_from: AccountId,
        token_to: AccountId,
        is_reversible: bool,
        in_token_rate: u32,
        out_token_rate: u32,
    ) -> u32;

    /// only pool creator or admin can withdraw token in pool
    /// if amount is Option::None, it means withdraw all
    fn withdraw_token_in_pool(
        &mut self,
        pool_id: PoolId,
        token_id: AccountId,
        amount: Option<U128>,
    );

    fn delete_pool(&mut self, pool_id: PoolId);
}

pub trait AdminAction {
    fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>);

    fn remove_whitelisted_tokens(&mut self, tokens: Vec<AccountId>);

    fn set_deposit_amount_of_pool_creation(&mut self, amount: U128);
    ///
    fn pause_contract(&mut self);
    ///
    fn resume_contract(&mut self);
}

pub trait AccountAction {
    fn withdraw_token_in_account(&mut self, account_id: AccountId);
}
