use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::account::Account;
use crate::conversion_pool::ConversionPool;
use crate::token_receiver::ConvertAction;
use crate::{FtMetaData, PoolId};

pub trait ConvertorViewer{

    fn minimum_received(&self, in_token_amount: U128, pool_id: PoolId)->U128;

    fn select_best_pool(&self, in_token: AccountId,out_token: AccountId,in_token_amount: U128 )->Option<(ConversionPool,U128)> ;
    // 根据token兑换方向获取pools
    fn get_pools_by_token_direction(&self, in_token: AccountId,out_token: AccountId )->Vec<ConversionPool>;

    fn get_whitelist(&self)->Vec<FtMetaData>;

    fn get_pools(&self, from_index: u64, limit: u64)->Vec<ConversionPool>;
}

pub trait PoolCreatorAction {
    fn create_pool(
        &mut self,
        token_from: AccountId,
        token_to: AccountId,
        // 是否可逆
        is_reversible: bool,
        // 汇率
        rate: u32,
        rate_decimal: u32
    )->u64;
}

pub trait AdminAction {
    /// Extend whitelisted tokens with new tokens. Only can be called by owner.
    fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>);
    /// Remove whitelisted token. Only can be called by owner.
    fn remove_whitelisted_tokens(&mut self, tokens: Vec<AccountId>);
}

pub trait FtOnTransferAction {
    fn single_pool_convert(convert_action: ConvertAction);
}
