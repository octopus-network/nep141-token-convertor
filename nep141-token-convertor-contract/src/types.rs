use crate::*;
use near_sdk::json_types::U64;
use near_sdk::AccountId;
use uint::construct_uint;

pub type PoolId = U64;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
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
