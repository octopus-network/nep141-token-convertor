use crate::conversion_pool::ConversionPool;
use crate::token_receiver::ConvertAction;
use crate::{FtMetaData, PoolId};
use near_sdk::{AccountId, Balance};

pub trait ConvertorViewer {
    fn get_whitelist(&self) -> Vec<FtMetaData>;

    fn get_pools(&self, from_index: u64, limit: u64) -> Vec<ConversionPool>;
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
        rate_decimal: u32,
    ) -> u64;

    fn remove_liquidity(&mut self, pool_id: PoolId, token_id: AccountId, amount: Balance);
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
