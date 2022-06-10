use crate::account::Account;
use crate::constants::{T_GAS_FOR_FT_TRANSFER, T_GAS_FOR_RESOLVE_TRANSFER};
use crate::events::{EventEmit, PoolEvent};
use crate::*;
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::ONE_YOCTO;
use std::ops::Mul;

// user convert a type of token into another in some pool
// user can specify except receive token id and amount.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ConvertAction {
    // pool id
    pub pool_id: PoolId,
    pub input_token_id: AccountId,
    pub input_token_amount: U128,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum TransferMessage {
    AddLiquidity { pool_id: PoolId },
    Convert { convert_action: ConvertAction },
}

#[near_bindgen]
impl FungibleTokenReceiver for TokenConvertor {
    /// Callback on receiving tokens by this contract.
    /// `msg` format must can be deserialize `TransferMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.assert_contract_is_not_paused();
        let transfer_message: TransferMessage =
            serde_json::from_str(msg.as_str()).expect("Invalid parameter 'msg' is attached.");
        let token_id = env::predecessor_account_id();
        match transfer_message {
            TransferMessage::AddLiquidity { pool_id } => {
                self.internal_use_pool(pool_id, |pool| {
                    assert_eq!(
                        sender_id, pool.creator,
                        "Only pool creator can deposit token into the pool."
                    );
                    pool.add_liquidity(&token_id, amount.0);
                });
                PoolEvent::UpdatePool {
                    pool: self.internal_get_pool(&pool_id).as_ref().unwrap(),
                }
                .emit();
            }
            TransferMessage::Convert { convert_action } => {
                assert_eq!(
                    token_id, convert_action.input_token_id,
                    "Received token '{}' does not match the token '{}' specified in attached 'msg'.",
                    token_id, convert_action.input_token_id
                );
                assert_eq!(
                    amount, convert_action.input_token_amount,
                    "Received amount '{}' does not match the amount '{}' specified in attached 'msg'.",
                    amount.0, convert_action.input_token_amount.0
                );
                let (receive_token_id, receive_token_amount) =
                    self.internal_convert(convert_action.pool_id, &token_id, amount.0);
                let pool = self.internal_get_pool(&convert_action.pool_id).unwrap();
                PoolEvent::UpdatePool { pool: &pool }.emit();
                self.internal_send_tokens(&sender_id, &receive_token_id, receive_token_amount);
            }
        }
        PromiseOrValue::Value(U128(0))
    }
}

#[near_bindgen]
impl TokenConvertor {
    pub(crate) fn internal_send_near(&self, receiver_id: AccountId, amount: Balance) -> Promise {
        Promise::new(receiver_id).transfer(amount)
    }

    pub(crate) fn internal_send_tokens(
        &mut self,
        receiver_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
    ) -> Promise {
        self.assert_storage_balance_bound_min(receiver_id);
        // account ft_transfer_lock plus one, it'll minus one when ft_transfer_resolved,
        // By this way, contract can avoid some methods executing between ft_transfer and ft_transfer_resolved
        self.internal_use_account(&receiver_id, |account| account.plus_ft_transfer_lock());

        ext_ft_core::ext(token_id.clone())
            .with_attached_deposit(ONE_YOCTO)
            .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER))
            .ft_transfer(receiver_id.clone(), U128(amount), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_RESOLVE_TRANSFER))
                    .ft_transfer_resolved(token_id.clone(), receiver_id.clone(), U128(amount)),
            )
    }

    #[private]
    pub fn ft_transfer_resolved(
        &mut self,
        token_id: AccountId,
        sender_id: AccountId,
        amount: U128,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Expect 1 promise result for sending token."
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                self.internal_use_account(&sender_id, |account| account.minus_ft_transfer_lock());
            }
            PromiseResult::Failed => {
                // This reverts the changes from withdraw function.
                // If account doesn't exit, deposits to the owner's account as lostfound.
                log!(
                    "Failed to transfer token '{}' for '{}'. Try to register the account in the token contract first.",
                    token_id,
                    sender_id
                );
                let mut account = self
                    .internal_get_account(&sender_id)
                    .unwrap_or(Account::new());
                account.deposit_token(&token_id, amount.0);
                account.minus_ft_transfer_lock();
                self.internal_save_account(&sender_id, account);
            }
        };
    }
}
