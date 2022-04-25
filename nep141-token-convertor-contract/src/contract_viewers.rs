use crate::*;
use crate::contract_interfacs::ConvertorViewer;
use crate::conversion_pool::ConversionPool;

#[near_bindgen]
impl ConvertorViewer for TokenConvertor {

    fn get_whitelist(&self) -> Vec<FtMetaData> {
        self.whitelisted_tokens.values().collect_vec()
    }

    fn get_pools(&self, from_index: u64, limit: u64)->Vec<ConversionPool> {
        (from_index..std::cmp::min(from_index+limit, self.pools.len()))
            .map(|index|self.internal_get_pool(index).unwrap())
            .collect_vec()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test_viewer {
    use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use crate::contract_interfacs::{AdminAction, ConvertorViewer, PoolCreatorAction};
    use crate::FtMetaData;
    use crate::test::{setup_contract, USDC, USDT};

}