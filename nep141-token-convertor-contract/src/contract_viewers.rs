use crate::*;
use crate::contract_interfacs::ConvertorViewer;
use crate::conversion_pool::ConversionPool;

#[near_bindgen]
impl ConvertorViewer for TokenConvertor {

    fn minimum_received(&self, in_token_amount: U128, pool_id: PoolId) -> U128 {
        let pool = self.internal_get_pool(pool_id).expect("no pool");
        return U128(pool.calculate_output_token_amount(in_token_amount.0));
    }


    fn select_best_pool(&self, in_token: AccountId,out_token: AccountId,in_token_amount: U128 )->Option<(ConversionPool,U128)> {
        let judge = |pool: &ConversionPool|->Option<u128>{
            if pool.in_token.eq(&in_token)&&pool.out_token.eq(&out_token) {
                let out_amount = pool.calculate_output_token_amount(in_token_amount.0);
                if out_amount<=pool.out_token_balance.0 {
                    return Option::Some(out_amount)
                }
            }

            if pool.reversible&&pool.in_token.eq(&out_token)&&pool.out_token.eq(&in_token) {
                let out_amount = pool.calculate_reverse_output_token_amount(in_token_amount.0);
                if out_amount<=pool.in_token_balance.0 {
                    return Option::Some(out_amount)
                }
            }
            return Option::None
        };
        let mut res: Option<(ConversionPool, U128)> = Option::None;
        for e in self.pools.iter() {
            let pool = e.into_current();
            let out_amount_option = judge(&pool);
            if out_amount_option.is_none() {continue}
            let out_amount = out_amount_option.unwrap();
            let tmp = res.clone().map(|e|e.1.0);

            if res.is_none() || tmp.unwrap()<=out_amount {
                res = Option::Some((pool.clone(),U128(out_amount)))
            }
        }
        return res
    }

    fn get_pools_by_token_direction(&self,
                                    in_token: AccountId,
                                    out_token: AccountId) -> Vec<ConversionPool> {
        self.pools.iter()
            .map(Pool::into_current)
            .filter(|e| { e.in_token.eq(&in_token) && e.out_token.eq(&out_token) })
            .collect_vec()
    }

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

    #[test]
    fn test_select_best() {
        let (mut context, mut contract, mut whitelist_amdmin) = setup_contract();
        context.predecessor_account_id(whitelist_amdmin.clone());
        contract.extend_whitelisted_tokens(vec![
            FtMetaData{ token_id: USDT , decimals: 6 },
            FtMetaData{ token_id: USDC, decimals: 6 }]);
        contract.create_pool(USDT,USDC,true,1000,3);
        contract.internal_use_pool(0,|pool|{
            pool.add_liquidity(&USDC, 3);
        });
        let option = contract.select_best_pool(USDT.clone(), USDC.clone(), U128(1));
        println!("{:?}", option);
    }

}