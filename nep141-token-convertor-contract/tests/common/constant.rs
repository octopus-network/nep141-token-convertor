use near_sdk::AccountId;
use std::convert::TryFrom;

pub fn string_to_account(name: &str) -> AccountId {
    AccountId::try_from(name.to_string()).unwrap()
}

pub fn convertor_contract_id() -> AccountId {
    string_to_account("convertor")
}

pub fn admin_id() -> AccountId {
    string_to_account("admin")
}
