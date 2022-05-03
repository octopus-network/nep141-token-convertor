use crate::conversion_pool::ConversionPool;
use crate::token_receiver::ConvertAction;
use crate::{FtMetaData, PoolId};
use near_sdk::json_types::U128;
use near_sdk::{AccountId };

pub trait ConvertorViewer {
    fn get_whitelist(&self) -> Vec<FtMetaData>;

    fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool>;

    fn get_creator_pools(&self, account_id: AccountId) -> Vec<ConversionPool>;
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

    fn withdraw_token_in_pool(&mut self, pool_id: PoolId, token_id: AccountId, amount: U128);

    fn delete_pool(&mut self, pool_id: PoolId);
}

pub trait AdminAction {
    fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>);

    fn remove_whitelisted_tokens(&mut self, tokens: Vec<AccountId>);

    fn set_pool_create_deposit_amount(&mut self, amount: U128);
}

pub trait AccountAction {
    fn withdraw_token_in_account(&mut self, account_id: AccountId);
}

pub trait FtOnTransferAction {
    fn single_pool_convert(convert_action: ConvertAction);
}
