use crate::*;
use near_sdk::AccountId;
use uint::construct_uint;

pub type PoolId = u32;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TokenDirectionKey(String);

impl TokenDirectionKey {
    pub fn new(from_token: &AccountId, to_token: &AccountId) -> Self {
        Self {
            0: format!("{}:{}", from_token.to_string(), to_token.to_string()),
        }
    }
}

#[derive(
    BorshDeserialize,
    BorshSerialize,
    Debug,
    Clone,
    Deserialize,
    Serialize,
    Eq,
    PartialOrd,
    PartialEq,
)]
#[serde(crate = "near_sdk::serde")]
pub struct FtMetaData {
    pub token_id: AccountId,
    pub decimals: u8,
}
