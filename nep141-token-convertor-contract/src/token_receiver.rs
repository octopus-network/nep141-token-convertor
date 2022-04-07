use std::ops::Mul;
use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use crate::account::Account;
use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use crate::constants::{T_GAS_FOR_FT_TRANSFER, T_GAS_FOR_RESOLVE_TRANSFER};
use crate::external_trait::ext_self;

// user convert a type of token into another in some pool
// user can specify except receive token id and amount.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ConvertAction {
    // pool id
    pub pool_id: u64,
    pub input_token_id: AccountId,
    pub input_token_amount: U128,
    // except receive token
    pub except_receive_token_id: AccountId,
    // except output token amount
    pub except_receive_token_amount: U128,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum TransferMessage {
    AddLiquidity {
        pool_id: u64,
    },
    Convert {
        convert_action: ConvertAction
    },
}

#[near_bindgen]
impl FungibleTokenReceiver for TokenConvertor {
    /// Callback on receiving tokens by this contract.
    /// `msg` format must can be deserialize `TokenReceiverMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        let transfer_message: TransferMessage = serde_json::from_str(msg.as_str())
            .expect("Fail to deserialize msg when ft_on_transfer.");
        let token_id = env::predecessor_account_id();
        match transfer_message {
            TransferMessage::AddLiquidity { pool_id } => {
                self.internal_use_pool(pool_id, |pool| {
                    pool.add_liquidity(&token_id, amount.0);
                });
            }
            TransferMessage::Convert { convert_action } => {
                assert_eq!(token_id, convert_action.input_token_id,
                           "transferred token id: {} not eq convert_action token id: {} ",
                           token_id, convert_action.input_token_id);
                assert_eq!(amount, convert_action.input_token_amount,
                           "transferred token amount: {} not eq convert_action token amount: {} ",
                           amount.0, convert_action.input_token_amount.0);
                let (receive_token_id, receive_token_amount) =
                    self.internal_convert(convert_action.pool_id,
                                          &token_id,
                                          amount.0);
                self.internal_send_tokens(
                    &sender_id,
                    &receive_token_id,
                    receive_token_amount);
            }
        }
        PromiseOrValue::Value(U128(0))
    }
}

#[near_bindgen]
impl TokenConvertor {
    pub(crate) fn internal_send_tokens(
        &self,
        sender_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
    ) -> Promise
    {
        ext_fungible_token::ft_transfer(
            sender_id.clone(),
            U128(amount),
            None,
            token_id.clone(),
            1,
            Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER),
        ).then(ext_self::ft_transfer_resolved(
            token_id.clone(),
            sender_id.clone(),
            U128(amount),
            env::current_account_id(),
            0,
            Gas::ONE_TERA.mul(T_GAS_FOR_RESOLVE_TRANSFER),
        ))
    }

    #[private]
    pub fn ft_transfer_resolved(
        &mut self,
        token_id: AccountId,
        sender_id: AccountId,
        amount: U128, )
    {
        assert_eq!(
            env::promise_results_count(),
            1,
            "expected 1 promise result from withdraw"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {}
            PromiseResult::Failed => {
                // This reverts the changes from withdraw function.
                // If account doesn't exit, deposits to the owner's account as lostfound.

                log!("Transfer token failed.Try to deposit token into account.");
                let mut account = self.internal_get_account(&sender_id)
                    .unwrap_or(Account::new());
                account.deposit_token(&token_id, amount.0);
                self.internal_save_account(&sender_id, account, false);
            }
        };
    }
}