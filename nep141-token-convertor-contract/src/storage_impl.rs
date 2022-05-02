use crate::account::Account;
use crate::*;
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::{assert_one_yocto, Promise};

#[near_bindgen]
impl StorageManagement for TokenConvertor {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        let attach_amount = env::attached_deposit();
        let account_id = account_id.unwrap_or(env::predecessor_account_id());
        let registration_only = registration_only.unwrap_or(false);
        let min_balance = self.storage_balance_bounds().min.0;
        let already_registered = self.accounts.contains_key(&account_id);
        let mut account = self
            .internal_get_account(&account_id)
            .unwrap_or(Account::new());
        assert!(
            already_registered && attach_amount >= min_balance,
            "ERR_DEPOSIT_LESS_THAN_REGISTER_NEED."
        );
        if registration_only {
            account.near_amount_for_storage += min_balance;
            let refund = attach_amount - min_balance;
            if refund > 0 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        } else {
            account.near_amount_for_storage += attach_amount;
            self.internal_save_account(&account_id, account, true);
        }

        return self.storage_balance_of(account_id).unwrap();
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        assert_one_yocto();
        let transfer_amount: u128 =
            self.internal_use_account(&env::predecessor_account_id(), true, |account| {
                let withdraw_amount = amount
                    .map(|e| e.0)
                    .unwrap_or(account.storage_available_balance());
                assert!(withdraw_amount <= account.storage_available_balance());
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
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        if let Some(account) = self.internal_get_account(&account_id) {
            // todo: can't force unregister now
            assert!(
                account.tokens.is_empty(),
                "ERR_STORAGE_UNREGISTER_TOKENS_NOT_EMPTY"
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
            min: U128(0),
            max: Option::None,
        };
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        match self.internal_get_account(&account_id) {
            None => Option::None,
            Some(account) => Option::Some(StorageBalance {
                total: U128(account.near_amount_for_storage),
                available: U128(account.storage_available_balance()),
            }),
        }
    }
}
