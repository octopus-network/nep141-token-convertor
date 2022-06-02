// use anyhow::anyhow;
// use near_sdk::AccountId;
// use near_sdk::json_types::U128;
// use nep141_token_convertor_contract::account::AccountView;
// use nep141_token_convertor_contract::conversion_pool::ConversionPool;
// use nep141_token_convertor_contract::FtMetaData;
// use nep141_token_convertor_contract::types::PoolId;
// use async_trait::async_trait;
//
// #[async_trait]
// pub trait AsyncConvertorViewer {
//     async fn get_whitelist(&self) -> Vec<FtMetaData>;
//
//     async fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool>;
//
//     async fn get_pools_by_creator(&self, account_id: AccountId) -> Vec<ConversionPool>;
//
//     /// storage fee need deposit = storage_balance_bounds.min - account.near_amount_for_storage
//     /// if account.near_amount_for_storage > storage_balance_bounds.min, it should return 0
//     async fn get_storage_fee_gap_of(&self, account_id: AccountId) -> U128;
//
//     async fn get_account(&self, account_id: near_sdk::AccountId) -> AccountView;
//
//     async fn is_contract_paused(&self) -> bool;
//
//     async fn get_deposit_amount_of_pool_creation(&self) -> U128;
// }
//
// #[async_trait]
// pub trait AsyncPoolCreatorAction {
//     async fn create_pool(
//         &mut self,
//         token_from: near_sdk::AccountId,
//         token_to: near_sdk::AccountId,
//         is_reversible: bool,
//         in_token_rate: u32,
//         out_token_rate: u32,
//     ) -> PoolId;
//
//     /// only pool creator or owner can withdraw token in pool
//     /// if amount is Option::None, it means withdraw all
//     async fn withdraw_token_in_pool(
//         &mut self,
//         pool_id: PoolId,
//         token_id: near_sdk::AccountId,
//         amount: Option<U128>,
//     );
//
//     async fn delete_pool(&mut self, pool_id: PoolId);
// }
//
// #[async_trait]
// pub trait AsyncOwnerAction {
//     async fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>);
//
//     async fn remove_whitelisted_tokens(&mut self, tokens: Vec<near_sdk::AccountId>);
//
//     async fn set_deposit_amount_of_pool_creation(&mut self, amount: U128);
//     ///
//     async fn pause_contract(&mut self);
//     ///
//     async fn resume_contract(&mut self);
// }
//
// #[async_trait]
// pub trait AsyncAccountAction {
//     async fn withdraw_token_in_account(&mut self, token_id: near_sdk::AccountId);
// }
