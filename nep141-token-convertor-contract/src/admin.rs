use crate::contract_interfaces::AdminAction;
use crate::types::FtMetaData;
use crate::*;

#[near_bindgen]
impl TokenConvertor {
    pub(crate) fn assert_admin_access(&self) {
        assert_eq!(
            self.admin,
            env::predecessor_account_id(),
            "require admin access permission."
        );
    }
}

#[near_bindgen]
impl AdminAction for TokenConvertor {
    fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>) {
        self.assert_admin_access();
        for token in tokens {
            self.whitelisted_tokens.insert(&token.token_id, &token);
        }
    }

    fn remove_whitelisted_tokens(&mut self, tokens: Vec<AccountId>) {
        self.assert_admin_access();
        for e in tokens {
            self.whitelisted_tokens.remove(&e);
        }
    }

    /// change deposit near amount when creating this pool
    fn set_pool_create_deposit_amount(&mut self, amount: U128) {
        self.assert_admin_access();
        self.create_pool_deposit = amount.0;
    }

    fn set_contract_is_paused(&mut self, is_paused: bool) {
        self.contract_is_paused = is_paused
    }
}
