use crate::account::Account;
use crate::constants::{PREPAY_STORAGE_FOR_REGISTERED, PREPAY_STORAGE_FOR_UNREGISTERED};
use crate::*;
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::{assert_one_yocto, Promise};

#[near_bindgen]
impl StorageManagement for TokenConvertor {
    /// if account_id is Option::None, it will be deposited for env::predecessor_account_id().
    /// if registration_only is true, the near tokens that exceed internal_get_storage_balance_min_bound will be refunded.
    /// if registration_only is false, all of the attached near tokens will be deposited.
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.assert_contract_is_not_paused();
        let attach_amount = env::attached_deposit();
        let account_id = account_id.unwrap_or(env::predecessor_account_id());
        let mut account = self
            .internal_get_account(&account_id)
            .unwrap_or(Account::new());
        let registration_only = registration_only.unwrap_or(false);
        let min_balance = self.internal_get_storage_balance_min_bound(&account_id);
        log!(
            "{} storage deposit {} yocto near, require at least deposit {} yocto near for storage now.",
            env::predecessor_account_id(),
            env::attached_deposit(),
            min_balance
        );
        assert!(
            attach_amount + account.near_amount_for_storage >= min_balance,
            "At least deposit {} yocto near.",
            min_balance - account.near_amount_for_storage
        );

        account.near_amount_for_storage += attach_amount;
        if registration_only {
            let refund = account.near_amount_for_storage - min_balance;
            account.near_amount_for_storage = min_balance;
            self.internal_save_account(&account_id, account);
            if refund > 0 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        } else {
            self.internal_save_account(&account_id, account);
        }

        return self.storage_balance_of(account_id).unwrap();
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        self.assert_contract_is_not_paused();
        assert_one_yocto();
        let transfer_amount: u128 =
            self.internal_use_account(&env::predecessor_account_id(), |account| {
                let withdraw_amount = amount
                    .map(|e| e.0)
                    .unwrap_or(account.available_storage_deposit());
                assert!(
                    withdraw_amount <= account.available_storage_deposit(),
                    "withdraw amount {}, but only available {}",
                    withdraw_amount,
                    account.available_storage_deposit()
                );
                account.near_amount_for_storage -= withdraw_amount;
                withdraw_amount
            });
        if transfer_amount > 0 {
            Promise::new(env::predecessor_account_id()).transfer(transfer_amount);
        }
        return self
            .storage_balance_of(env::predecessor_account_id())
            .unwrap();
    }

    #[allow(unused_variables)]
    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        self.assert_contract_is_not_paused();
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        assert!(
            !self.internal_check_ft_transfer_is_lock(&account_id),
            "Fail to storage_unregister because still having ft_transfer not resolved."
        );
        if let Some(account) = self.internal_get_account(&account_id) {
            assert!(
                account.tokens.is_empty(),
                "Fail to storage_unregister because still having token in account."
            );
            self.accounts.remove(&account_id);
            if account.near_amount_for_storage > 0 {
                Promise::new(account_id.clone()).transfer(account.near_amount_for_storage);
            }
            true
        } else {
            false
        }
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        return StorageBalanceBounds {
            min: U128(PREPAY_STORAGE_FOR_UNREGISTERED as u128 * env::storage_byte_cost()),
            max: Option::None,
        };
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        match self.internal_get_account(&account_id) {
            None => Option::None,
            Some(account) => Option::Some(StorageBalance {
                total: U128(account.near_amount_for_storage),
                available: U128(account.available_storage_deposit()),
            }),
        }
    }
}

impl TokenConvertor {
    pub(crate) fn internal_get_storage_balance_min_bound(&self, account_id: &AccountId) -> u128 {
        let account = self.internal_get_account(account_id);
        let min_usage = if account.is_some() {
            // besides actually usage, need to add maximum storage cost of all change methods
            account.unwrap().storage_usage() + PREPAY_STORAGE_FOR_REGISTERED
        } else {
            PREPAY_STORAGE_FOR_UNREGISTERED
        };
        return min_usage as u128 * env::storage_byte_cost();
    }
}
