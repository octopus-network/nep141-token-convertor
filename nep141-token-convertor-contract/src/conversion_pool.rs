use crate::*;
use crate::contract_interfacs::{PoolCreatorAction};
use crate::types::U256;

#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Pool {
    Current(ConversionPool)
}

impl Pool {
    pub fn into_current(self) ->ConversionPool {
        match self {
            Pool::Current(pool) => {pool}
        }
    }
}

impl From<ConversionPool> for Pool {
    fn from(pool: ConversionPool) -> Self {
        Pool::Current(pool)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ConversionPool {
    pub id: u64,
    pub in_token: AccountId,
    pub in_token_balance: U128,
    pub out_token: AccountId,
    pub out_token_balance: U128,
    // reversible or not
    pub reversible: bool,
    // rate for convertor
    pub rate: u32,
    pub rate_decimal: u32
}

impl ConversionPool {

    pub fn new(id: u64,
               from_token: AccountId,
               to_token: AccountId,
               reversible: bool,
               rate: u32,
               rate_decimal: u32) ->Self{
        Self {
            id,
            in_token: from_token,
            in_token_balance: U128(0),
            out_token: to_token,
            out_token_balance: U128(0),
            reversible,
            rate,
            rate_decimal
        }
    }

    ///
    pub fn convert(&mut self,
                   input_token_id: &AccountId,
                   input_token_amount: Balance)->(AccountId,Balance) {
        self.check_input_token_legal(input_token_id);
        return if input_token_id.eq(&self.in_token) {
            let output_token_amount = self.calculate_output_token_amount(input_token_amount);
            assert!(self.out_token_balance.0 >= output_token_amount,
                    "Fail to convert,pool to_token balance {} less than output token amount: {}",
                    self.out_token_balance.0,
                    output_token_amount);
            self.deposit_from_token(input_token_amount);
            self.withdraw_to_token(output_token_amount);
            (self.out_token.clone(), output_token_amount)
        } else {
            let output_token_amount = self.calculate_reverse_output_token_amount(input_token_amount);
            assert!(self.in_token_balance.0 >= output_token_amount,
                    "Fail to convert,pool from_token balance {} less than output token amount: {}",
                    self.in_token_balance.0,
                    output_token_amount);
            self.deposit_to_token(input_token_amount);
            self.withdraw_from_token(output_token_amount);
            (self.in_token.clone(), output_token_amount)
        }
    }

    pub fn add_liquidity(&mut self, token_id: &AccountId,token_balance: Balance ) {
        self.check_input_token_legal(token_id);
        if token_id.eq(&self.in_token) {
            self.deposit_from_token(token_balance);
        }
        else {
            self.deposit_to_token(token_balance);
        };
    }

    pub fn calculate_output_token_amount(&self, input_token_amount: Balance)->Balance {
        (U256::from(input_token_amount) * self.rate / ((10 as u64).pow(self.rate_decimal)))
            // Panics if the number is larger than 2^128.
            .as_u128()
    }

    pub fn calculate_reverse_output_token_amount(&self, token_amount: Balance)->Balance {
        (U256::from(token_amount)/self.rate/((10 as u64).pow(self.rate_decimal))).as_u128()
    }

    fn deposit_from_token(&mut self, deposit_balance: Balance) {
        let new_balance = self.in_token_balance.0
            .checked_add(deposit_balance)
            .expect("Fail to deposit_from_token,token balance overflow");
        self.in_token_balance = U128(new_balance);
    }

    fn deposit_to_token(&mut self, deposit_balance: Balance) {
        let new_balance = self.out_token_balance.0
            .checked_add(deposit_balance)
            .expect("Fail to deposit_to_token,token balance overflow");
        self.out_token_balance =U128(new_balance);
    }

    fn withdraw_from_token(&mut self, withdraw_amount: Balance) {
        assert!(self.in_token_balance.0 >=withdraw_amount,
                "Fail to withdraw_from_token, pool balance not enough!");
        self.in_token_balance = U128(self.in_token_balance.0-withdraw_amount);
    }

    fn withdraw_to_token(&mut self, withdraw_amount: Balance) {
        assert!(self.in_token_balance.0 >=withdraw_amount,
                "Fail to withdraw_from_token, pool balance not enough!");
        self.in_token_balance = U128(self.in_token_balance.0-withdraw_amount);
    }


    fn check_input_token_legal(&self,token_id: &AccountId) {
        assert!(
            token_id.eq(&self.out_token)||token_id.eq(&self.in_token),
            "illegal input token: {},only accept {} or {}.",
            token_id,
            self.in_token,
            self.out_token
        );
        assert!(
            token_id.eq(&self.out_token)||self.reversible,
            "illegal input token {},only accept from token when pool is reversible",
            token_id);
    }
}
#[near_bindgen]
impl TokenConvertor {

    #[private]
    pub(crate) fn internal_convert(&mut self,
                                   pool_id: PoolId,
                                   input_token_id: &AccountId,
                                   token_amount: Balance)->(AccountId,Balance) {
        let mut pool = self.internal_get_pool(pool_id).expect("no such pool");
        let x = pool.convert(input_token_id, token_amount);
        self.internal_save_pool(pool_id, &pool.into());
        return x;

        // return self.internal_use_pool(pool_id, |pool|{
        //     return pool.convert(input_token_id,token_amount);
        // });
    }

    #[private]
    pub(crate) fn internal_use_pool<F,R>(&mut self, pool_id: PoolId,mut f: F)->R
        where F: FnMut(&mut ConversionPool)->R {
        let mut pool = self.internal_get_pool(pool_id).expect("No such pool");
        let r = f(&mut pool);
        self.internal_save_pool(pool_id,&Pool::Current(pool));
        r
    }

    #[private]
    pub(crate) fn internal_get_pool(&self, pool_id: PoolId) -> Option<ConversionPool>{
        return self.pools.get(pool_id)
            .map(|pool|pool.into_current());
    }

    #[private]
    pub(crate) fn internal_save_pool(&mut self, pool_id: PoolId, pool: &Pool) {
        self.pools.replace(pool_id,&pool);
    }

    #[private]
    pub fn assert_token_in_whitelist(&self, token: &AccountId) {
        assert!(self.whitelisted_tokens.get(token).is_some(),"token {} is not in whitelist!",token);
    }

}

#[near_bindgen]
impl PoolCreatorAction for TokenConvertor {

    #[payable]
    fn create_pool(&mut self,
                   from_token: AccountId,
                   to_token: AccountId,
                   is_reversible: bool,
                   rate: u32,
                   rate_decimal:u32 )->u64 {
        assert!(!from_token.eq(&to_token),"You can't create pool for same token");
        self.assert_token_in_whitelist(&from_token);
        self.assert_token_in_whitelist(&to_token);
        assert_eq!(
            self.whitelisted_tokens.get(&from_token).unwrap().decimals,
            self.whitelisted_tokens.get(&to_token).unwrap().decimals,
            "tokens in a pool should have same decimals."
        );
        let prev_storage = env::storage_usage();
        let id = self.pools.len() as u64;
        self.pools.push(&Pool::Current(ConversionPool::new(
            id.clone(), from_token.clone(), to_token.clone(), is_reversible, rate, rate_decimal)));
        self.internal_storage_deposit(prev_storage);
        id
    }
}