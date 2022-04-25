use crate::contract_interfacs::PoolCreatorAction;
use crate::types::U256;
use crate::*;
use near_sdk::assert_one_yocto;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Pool {
    Current(ConversionPool),
}

impl Pool {
    pub fn into_current(self) -> ConversionPool {
        match self {
            Pool::Current(pool) => pool,
        }
    }
}

impl From<ConversionPool> for Pool {
    fn from(pool: ConversionPool) -> Self {
        Pool::Current(pool)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ConversionPool {
    pub id: u64,
    pub creator: AccountId,
    pub in_token: AccountId,
    pub in_token_balance: U128,
    pub out_token: AccountId,
    pub out_token_balance: U128,
    // reversible or not
    pub reversible: bool,
    // rate for convertor,
    // for example: if 1 wNear = 0.9stNear，
    // it should set in_token_rate = 10, out_token_rate = 9
    pub in_token_rate: u32,
    pub out_token_rate: u32,
}

impl ConversionPool {
    pub fn new(
        id: u64,
        creator: AccountId,
        in_token: AccountId,
        out_token: AccountId,
        reversible: bool,
        in_token_rate: u32,
        out_token_rate: u32,
    ) -> Self {
        Self {
            id,
            creator,
            in_token,
            in_token_balance: U128(0),
            out_token,
            out_token_balance: U128(0),
            reversible,
            in_token_rate,
            out_token_rate,
        }
    }

    ///
    pub fn convert(
        &mut self,
        input_token_id: &AccountId,
        input_token_amount: Balance,
    ) -> (AccountId, Balance) {
        self.check_input_token_legal(input_token_id);
        return if input_token_id.eq(&self.in_token) {
            let output_token_amount = self.calculate_output_token_amount(input_token_amount);
            assert!(
                self.out_token_balance.0 >= output_token_amount,
                "Fail to convert,pool to_token balance {} less than output token amount: {}",
                self.out_token_balance.0,
                output_token_amount
            );
            self.deposit_from_token(input_token_amount);
            self.withdraw_out_token(output_token_amount);
            (self.out_token.clone(), output_token_amount)
        } else {
            let output_token_amount =
                self.calculate_reverse_output_token_amount(input_token_amount);
            assert!(
                self.in_token_balance.0 >= output_token_amount,
                "Fail to convert,pool from_token balance {} less than output token amount: {}",
                self.in_token_balance.0,
                output_token_amount
            );
            self.deposit_to_token(input_token_amount);
            self.withdraw_in_token(output_token_amount);
            (self.in_token.clone(), output_token_amount)
        };
    }

    pub fn add_liquidity(&mut self, token_id: &AccountId, token_balance: Balance) {
        self.check_input_token_legal(token_id);
        if token_id.eq(&self.in_token) {
            self.deposit_from_token(token_balance);
        } else {
            self.deposit_to_token(token_balance);
        };
    }

    pub fn calculate_output_token_amount(&self, token_amount: Balance) -> Balance {
        (U256::from(token_amount) * U256::from(self.in_token_rate) / U256::from(self.in_token_rate))
            .as_u128()
    }

    pub fn calculate_reverse_output_token_amount(&self, token_amount: Balance) -> Balance {
        (U256::from(token_amount) * U256::from(self.out_token_rate)
            / U256::from(self.in_token_rate))
        .as_u128()
    }

    fn deposit_from_token(&mut self, deposit_balance: Balance) {
        let new_balance = self
            .in_token_balance
            .0
            .checked_add(deposit_balance)
            .expect("Fail to deposit_from_token,token balance overflow");
        self.in_token_balance = U128(new_balance);
    }

    fn deposit_to_token(&mut self, deposit_balance: Balance) {
        let new_balance = self
            .out_token_balance
            .0
            .checked_add(deposit_balance)
            .expect("Fail to deposit_to_token,token balance overflow");
        self.out_token_balance = U128(new_balance);
    }

    fn withdraw_in_token(&mut self, withdraw_amount: Balance) {
        assert!(
            self.in_token_balance.0 >= withdraw_amount,
            "Fail to withdraw_from_token, pool balance not enough!"
        );
        self.in_token_balance = U128(self.in_token_balance.0 - withdraw_amount);
    }

    fn withdraw_out_token(&mut self, withdraw_amount: Balance) {
        assert!(
            self.in_token_balance.0 >= withdraw_amount,
            "Fail to withdraw_from_token, pool balance not enough!"
        );
        self.in_token_balance = U128(self.in_token_balance.0 - withdraw_amount);
    }

    fn check_input_token_legal(&self, token_id: &AccountId) {
        assert!(
            token_id.eq(&self.out_token) || token_id.eq(&self.in_token),
            "illegal input token: {},only accept {} or {}.",
            token_id,
            self.in_token,
            self.out_token
        );
        assert!(
            token_id.eq(&self.out_token) || self.reversible,
            "illegal input token {},only accept from token when pool is reversible",
            token_id
        );
    }
}
#[near_bindgen]
impl TokenConvertor {
    #[private]
    pub(crate) fn internal_convert(
        &mut self,
        pool_id: PoolId,
        input_token_id: &AccountId,
        token_amount: Balance,
    ) -> (AccountId, Balance) {
        let mut pool = self.internal_get_pool(pool_id).expect("no such pool");
        let x = pool.convert(input_token_id, token_amount);
        self.internal_save_pool(pool_id, &pool.into());
        return x;

        // return self.internal_use_pool(pool_id, |pool|{
        //     return pool.convert(input_token_id,token_amount);
        // });
    }

    #[private]
    pub(crate) fn internal_use_pool<F, R>(&mut self, pool_id: PoolId, mut f: F) -> R
    where
        F: FnMut(&mut ConversionPool) -> R,
    {
        let mut pool = self.internal_get_pool(pool_id).expect("No such pool");
        let r = f(&mut pool);
        self.internal_save_pool(pool_id, &Pool::Current(pool));
        r
    }

    #[private]
    pub(crate) fn internal_get_pool(&self, pool_id: PoolId) -> Option<ConversionPool> {
        return self.pools.get(pool_id).map(|pool| pool.into_current());
    }

    #[private]
    pub(crate) fn internal_save_pool(&mut self, pool_id: PoolId, pool: &Pool) {
        self.pools.replace(pool_id, &pool);
    }

    #[private]
    pub fn assert_token_in_whitelist(&self, token: &AccountId) {
        assert!(
            self.whitelisted_tokens.get(token).is_some(),
            "token {} is not in whitelist!",
            token
        );
    }
}

#[near_bindgen]
impl PoolCreatorAction for TokenConvertor {
    #[payable]
    fn create_pool(
        &mut self,
        in_token: AccountId,
        out_token: AccountId,
        is_reversible: bool,
        in_token_rate: u32,
        out_token_rate: u32,
    ) -> u64 {
        assert!(
            !in_token.eq(&out_token),
            "You can't create pool for same token"
        );
        self.assert_token_in_whitelist(&in_token);
        self.assert_token_in_whitelist(&out_token);
        assert_eq!(
            self.whitelisted_tokens.get(&in_token).unwrap().decimals,
            self.whitelisted_tokens.get(&out_token).unwrap().decimals,
            "tokens in a pool should have same decimals."
        );
        let prev_storage = env::storage_usage();
        let id = self.pools.len() as u64;
        self.pools.push(&Pool::Current(ConversionPool::new(
            id.clone(),
            env::predecessor_account_id(),
            in_token.clone(),
            out_token.clone(),
            is_reversible,
            in_token_rate,
            out_token_rate,
        )));
        self.internal_storage_deposit(prev_storage);
        id
    }

    #[payable]
    fn remove_liquidity(&mut self, pool_id: PoolId, token_id: AccountId, amount: Balance) {
        assert_one_yocto();
        self.internal_use_pool(pool_id, |pool| {
            assert_eq!(
                pool.creator,
                env::predecessor_account_id(),
                "Only creator can remove liquidity."
            );
            assert!(
                token_id == pool.in_token || token_id == pool.out_token,
                "Illegal token id {}",
                token_id
            );
            if token_id == pool.in_token {
                // if fail, it should panic
                pool.withdraw_in_token(amount);
            } else {
                pool.withdraw_out_token(amount);
            }
        });
        // pool should finish withdraw here
        self.internal_send_tokens(&env::predecessor_account_id(), &token_id, amount);
    }
}
