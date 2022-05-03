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

    fn get_creator_pools(&self, account_id: AccountId) -> Vec<ConversionPool> {
        self.pools
            .iter()
            .map(|e| e.1.into_current())
            .filter(|e| e.creator == account_id)
            .collect_vec()
    }

    fn get_account_storage_debt(&self, account_id: AccountId) -> U128 {
        U128::from(
            self.internal_get_account(&account_id)
                .expect("no such account")
                .storage_debt(),
        )
    }

    fn get_account(&self, account_id: AccountId) -> AccountView {
        self.internal_get_account(&account_id)
            .expect("no such account")
            .into()
    }

    fn is_contract_paused(&self) -> bool {
        self.contract_is_paused
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test_viewer {
    use crate::contract_interfaces::{AdminAction, ConvertorViewer, PoolCreatorAction};
    use crate::test::{setup_contract, USDC, USDT};
    use crate::FtMetaData;
    use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
}
