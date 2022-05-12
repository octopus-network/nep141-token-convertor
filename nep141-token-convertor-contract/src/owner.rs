use crate::contract_interfaces::OwnerAction;
use crate::types::FtMetaData;
use crate::*;
use near_contract_standards::upgrade::Ownable;

#[near_bindgen]
impl Ownable for TokenConvertor {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}

#[near_bindgen]
impl OwnerAction for TokenConvertor {
    fn extend_whitelisted_tokens(&mut self, tokens: Vec<FtMetaData>) {
        self.assert_owner();
        for token in tokens {
            self.whitelisted_tokens.insert(&token.token_id, &token);
        }
    }

    fn remove_whitelisted_tokens(&mut self, tokens: Vec<AccountId>) {
        self.assert_owner();
        for e in tokens {
            self.whitelisted_tokens.remove(&e);
        }
    }

    /// change deposit near amount when creating this pool
    fn set_deposit_amount_of_pool_creation(&mut self, amount: U128) {
        self.assert_owner();
        self.create_pool_deposit = amount.0;
    }

    fn pause_contract(&mut self) {
        self.assert_owner();
        assert!(!self.contract_is_paused, "Contract is already paused.");
        self.contract_is_paused = true;
    }

    fn resume_contract(&mut self) {
        self.assert_owner();
        assert!(self.contract_is_paused, "Contract is already active.");
        self.contract_is_paused = false;
    }
}
