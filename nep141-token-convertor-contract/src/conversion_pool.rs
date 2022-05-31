use crate::contract_interfaces::PoolCreatorAction;
use crate::events::{EventEmit, PoolEvent};
use crate::types::U256;
use crate::*;
use near_sdk::assert_one_yocto;
use near_sdk::json_types::U64;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VPool {
    Current(ConversionPool),
}

impl VPool {
    pub fn into_current(self) -> ConversionPool {
        match self {
            VPool::Current(pool) => pool,
        }
    }
}

impl From<ConversionPool> for VPool {
    fn from(pool: ConversionPool) -> Self {
        VPool::Current(pool)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ConversionPool {
    pub id: PoolId,
    pub creator: AccountId,
    pub in_token: AccountId,
    pub in_token_balance: U128,
    pub out_token: AccountId,
    pub out_token_balance: U128,
    /// reversible or not
    pub reversible: bool,
    /// rate for convertor,
    /// for example: if 1 wNear = 0.9stNear，
    /// it should set in_token_rate = 10, out_token_rate = 9
    pub in_token_rate: u32,
    pub out_token_rate: u32,
    /// deposit near amount when creating this pool
    pub deposit_near_amount: U128,
}

impl ConversionPool {
    pub fn new(
        id: PoolId,
        creator: AccountId,
        in_token: AccountId,
        out_token: AccountId,
        reversible: bool,
        in_token_rate: u32,
        out_token_rate: u32,
        deposit_near_amount: U128,
    ) -> Self {
        assert!(
            in_token_rate > 0 && out_token_rate > 0,
            "Both scale factors should be greater than 0."
        );
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
            deposit_near_amount,
        }
    }

    /// use a pool to convert
    /// if input token id equal pool's in_token, then it will convert input token into out_token
    /// if input token id equal pool's out_token, then it will convert input token into in_token
    pub fn convert(
        &mut self,
        input_token_id: &AccountId,
        input_token_amount: Balance,
    ) -> (AccountId, Balance) {
        self.check_input_token_legal_when_converting(input_token_id);
        return if input_token_id.eq(&self.in_token) {
            let output_token_amount = self.calculate_output_token_amount(input_token_amount);
            assert!(
                self.out_token_balance.0 >= output_token_amount,
                "Failed to convert. The balance of 'to_token' in the pool ({}) is less than expected amount: {}",
                self.out_token_balance.0,
                output_token_amount
            );
            self.deposit_from_token(input_token_amount);
            self.withdraw_out_token(Option::Some(output_token_amount));
            (self.out_token.clone(), output_token_amount)
        } else {
            let output_token_amount =
                self.calculate_reverse_output_token_amount(input_token_amount);
            assert!(
                self.in_token_balance.0 >= output_token_amount,
                "Failed to convert. The balance of 'from_token' in the pool ({}) is less than expected amount: {}",
                self.in_token_balance.0,
                output_token_amount
            );
            self.deposit_to_token(input_token_amount);
            self.withdraw_in_token(Option::Some(output_token_amount));
            (self.in_token.clone(), output_token_amount)
        };
    }

    pub fn add_liquidity(&mut self, token_id: &AccountId, token_balance: Balance) {
        self.check_input_token_legal_when_adding_liquidity(token_id);
        if token_id.eq(&self.in_token) {
            self.deposit_from_token(token_balance);
        } else {
            self.deposit_to_token(token_balance);
        };
    }

    /// calculate token amount when convert in_token into out_token
    pub fn calculate_output_token_amount(&self, token_amount: Balance) -> Balance {
        (U256::from(token_amount) * U256::from(self.out_token_rate)
            / U256::from(self.in_token_rate))
        .as_u128()
    }

    /// calculate token amount when convert out_token into in_token
    pub fn calculate_reverse_output_token_amount(&self, token_amount: Balance) -> Balance {
        (U256::from(token_amount) * U256::from(self.in_token_rate)
            / U256::from(self.out_token_rate))
        .as_u128()
    }

    fn deposit_from_token(&mut self, deposit_balance: Balance) {
        let new_balance = self
            .in_token_balance
            .0
            .checked_add(deposit_balance)
            .expect("Failed to deposit. Token balance overflowed.");
        self.in_token_balance = U128(new_balance);
    }

    fn deposit_to_token(&mut self, deposit_balance: Balance) {
        let new_balance = self
            .out_token_balance
            .0
            .checked_add(deposit_balance)
            .expect("Failed to deposit. Token balance overflowed.");
        self.out_token_balance = U128(new_balance);
    }

    /// if withdraw_amount is none,it means withdraw all tokens.
    fn withdraw_in_token(&mut self, withdraw_amount: Option<Balance>) -> Balance {
        return match withdraw_amount {
            None => {
                let amount = self.in_token_balance.0;
                self.in_token_balance = U128(0);
                amount
            }
            Some(amount) => {
                assert!(
                    self.in_token_balance.0 >= amount,
                    "Failed to withdraw. Available balance in the pool is not enough."
                );
                self.in_token_balance = U128(self.in_token_balance.0 - amount);
                amount
            }
        };
    }

    /// if withdraw_amount is none,it means withdraw all tokens.
    fn withdraw_out_token(&mut self, withdraw_amount: Option<Balance>) -> Balance {
        return match withdraw_amount {
            None => {
                let amount = self.out_token_balance.0;
                self.out_token_balance = U128(0);
                amount
            }
            Some(amount) => {
                assert!(
                    self.out_token_balance.0 >= amount,
                    "Failed to withdraw. Available balance in the pool is not enough."
                );
                self.out_token_balance = U128(self.out_token_balance.0 - amount);
                amount
            }
        };
    }

    fn check_input_token_legal_when_adding_liquidity(&self, token_id: &AccountId) {
        // token must be out_token or in_token
        assert!(
            token_id.eq(&self.out_token) || token_id.eq(&self.in_token),
            "Invalid input token: '{}'. Only accept '{}' or '{}'.",
            token_id,
            self.in_token,
            self.out_token
        );
        // token can only be out_token unless pool's reversible is true
        assert!(
            token_id.eq(&self.out_token) || self.reversible,
            "Invalid input token '{}'. Only accept '{}' if the pool is not reversible.",
            token_id,
            self.out_token
        );
    }

    fn check_input_token_legal_when_converting(&self, token_id: &AccountId) {
        // token must be out_token or in_token
        assert!(
            token_id.eq(&self.out_token) || token_id.eq(&self.in_token),
            "Invalid input token: '{}'. Only accept '{}' or '{}'.",
            token_id,
            self.in_token,
            self.out_token
        );
        // token can only be in_token unless pool's reversible is true
        assert!(
            token_id.eq(&self.in_token) || self.reversible,
            "Invalid input token '{}'. Only accept '{}' if the pool is not reversible.",
            token_id,
            self.in_token
        );
    }
}

impl TokenConvertor {
    pub(crate) fn internal_convert(
        &mut self,
        pool_id: PoolId,
        input_token_id: &AccountId,
        token_amount: Balance,
    ) -> (AccountId, Balance) {
        return self.internal_use_pool(pool_id, |pool| {
            return pool.convert(input_token_id, token_amount);
        });
    }

    pub(crate) fn internal_assign_pool_id(&mut self) -> PoolId {
        self.pool_id += 1;
        return U64(self.pool_id);
    }

    pub(crate) fn internal_delete_pool(&mut self, pool_id: &PoolId) {
        let pool = self
            .internal_get_pool(&pool_id)
            .expect(format!("Pool '{}' is not existed.", pool_id.0).as_str());
        assert_eq!(
            pool.in_token_balance.0, 0,
            "Failed to delete pool '{}'. All of the 'in token' in the pool must be withdrawn first.",
            pool_id.0
        );
        assert_eq!(
            pool.out_token_balance.0, 0,
            "Failed to delete pool '{}'. All of the 'out token' in the pool must be withdrawn first.",
            pool_id.0
        );
        self.pools.remove(pool_id);
        log!(
            "Pool '{}' is deleted by '{}'.",
            pool_id.0,
            env::predecessor_account_id()
        )
    }

    pub(crate) fn internal_use_pool<F, R>(&mut self, pool_id: PoolId, mut f: F) -> R
    where
        F: FnMut(&mut ConversionPool) -> R,
    {
        let mut pool = self.internal_get_pool(&pool_id).expect("No such pool.");
        let r = f(&mut pool);
        self.internal_save_pool(pool_id, &pool.into());
        r
    }

    pub(crate) fn internal_get_pool(&self, pool_id: &PoolId) -> Option<ConversionPool> {
        // return self.pools.get(&pool_id)
        return self.pools.get(pool_id).map(|pool| pool.into_current());
    }

    pub(crate) fn internal_save_pool(&mut self, pool_id: PoolId, pool: &VPool) {
        self.pools.insert(&pool_id, &pool);
        // self.pools.replace(pool_id, &pool);
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
    ) -> PoolId {
        self.assert_contract_is_not_paused();
        assert!(
            !in_token.eq(&out_token),
            "Can not create pool for two same tokens."
        );
        self.assert_create_pool_deposit_amount();
        self.assert_token_in_whitelist(&in_token);
        self.assert_token_in_whitelist(&out_token);
        assert_eq!(
            self.whitelisted_tokens.get(&in_token).unwrap().decimals,
            self.whitelisted_tokens.get(&out_token).unwrap().decimals,
            "Tokens in a pool should have the same decimals."
        );
        let id = self.internal_assign_pool_id();
        self.pools.insert(
            &id,
            &VPool::Current(ConversionPool::new(
                id.clone(),
                env::predecessor_account_id(),
                in_token.clone(),
                out_token.clone(),
                is_reversible,
                in_token_rate,
                out_token_rate,
                U128(env::attached_deposit()),
            )),
        );
        PoolEvent::CreatePool {
            pool: self.internal_get_pool(&id).as_ref().unwrap(),
        }
        .emit();
        id
    }

    #[payable]
    fn withdraw_token_in_pool(
        &mut self,
        pool_id: PoolId,
        token_id: AccountId,
        amount: Option<U128>,
    ) {
        self.assert_contract_is_not_paused();
        assert_one_yocto();
        let owner = self.owner.clone();
        let (creator, withdraw_amount) = self.internal_use_pool(pool_id, |pool| {
            assert!(
                pool.creator.eq(&env::predecessor_account_id())
                    || owner.eq(&env::predecessor_account_id()),
                "Only contract owner or pool creator can remove liquidity from the pool."
            );
            assert!(
                token_id == pool.in_token || token_id == pool.out_token,
                "Invalid token '{}'. Only '{}' or '{}' can be withdrawn.",
                token_id,
                pool.in_token,
                pool.out_token
            );
            return (
                pool.creator.clone(),
                if token_id == pool.in_token {
                    pool.withdraw_in_token(amount.map(|e| e.0))
                } else {
                    pool.withdraw_out_token(amount.map(|e| e.0))
                },
            );
        });
        PoolEvent::UpdatePool {
            pool: self.internal_get_pool(&pool_id).as_ref().unwrap(),
        }
        .emit();
        // pool should finish withdraw here
        if withdraw_amount> 0 {
            self.internal_send_tokens(&creator, &token_id, withdraw_amount);
        }
    }

    #[payable]
    fn delete_pool(&mut self, pool_id: PoolId) {
        self.assert_contract_is_not_paused();
        assert_one_yocto();
        let pool = self.internal_get_pool(&pool_id).expect("No such pool.");
        assert!(
            env::predecessor_account_id() == pool.creator
                || env::predecessor_account_id() == self.owner,
            "Only contract owner or pool creator can delete the pool."
        );
        self.internal_delete_pool(&pool_id);
        PoolEvent::DeletePool { pool_id: &pool_id }.emit();
        if pool.deposit_near_amount.0 > 0 {
            self.internal_send_near(pool.creator.clone(), pool.deposit_near_amount.0);
        }
    }
}
