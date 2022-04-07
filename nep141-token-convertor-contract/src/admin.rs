use std::collections::HashSet;
use crate::contract_interfacs::AdminAction;
use crate::*;
use crate::types::FtMetaData;

#[near_bindgen]
impl TokenConvertor {
    #[private]
    pub fn assert_admin_access(&self) {
       assert_eq!(self.admin, env::predecessor_account_id(),"require admin access permission.");
    }
}

#[near_bindgen]
impl AdminAction for TokenConvertor {

    fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>) {
        // self.assert_admin_access();
        for token in tokens {
            self.whitelisted_tokens.insert(&token.token_id,&token);
        }
    }

    fn remove_whitelisted_tokens(&mut self, tokens: Vec<AccountId>) {
        self.assert_admin_access();
        for e in tokens {
            self.whitelisted_tokens.remove(&e);

        }
    }
}