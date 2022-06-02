use crate::conversion_pool::ConversionPool;
use crate::serde_json::Value;
use crate::PoolId;
use near_sdk::log;
use near_sdk::serde::Serialize;
use near_sdk::serde_json::json;

pub const EVENT_STANDARD: &str = "convertor";
pub const EVENT_STANDARD_VERSION: &str = "1.0.0";

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "pool_event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum PoolEvent<'a> {
    CreatePool { pool: &'a ConversionPool },
    UpdatePool { pool: &'a ConversionPool },
    DeletePool { pool_id: &'a PoolId },
}

pub trait EventEmit {
    fn emit(&self)
    where
        Self: Sized + Serialize,
    {
        emit_event(&self);
    }
}

impl EventEmit for PoolEvent<'_> {}

// Emit event that follows NEP-297 standard: https://nomicon.io/Standards/EventsFormat
// Arguments
// * `standard`: name of standard, e.g. nep171
// * `version`: e.g. 1.0.0
// * `event`: type of the event, e.g. nft_mint
// * `data`: associate event data. Strictly typed for each set {standard, version, event} inside corresponding NEP
pub(crate) fn emit_event<T: ?Sized + Serialize>(data: &T) {
    let mut result = json!(data);
    let map = result.as_object_mut().unwrap();
    map.insert(
        "standard".to_string(),
        Value::String(EVENT_STANDARD.to_string()),
    );
    map.insert(
        "version".to_string(),
        Value::String(EVENT_STANDARD_VERSION.to_string()),
    );

    log!(format!("EVENT_JSON:{}", result.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{usdc, usdt};
    use near_sdk::json_types::U128;
    use near_sdk::json_types::U64;
    use near_sdk::test_utils;
    use near_sdk::test_utils::test_env::bob;

    #[test]
    fn test_pool_event() {
        PoolEvent::CreatePool {
            pool: &ConversionPool {
                id: U64(1),
                creator: bob(),
                in_token: usdc(),
                in_token_balance: U128(1),
                out_token: usdt(),
                out_token_balance: U128(1),
                reversible: false,
                in_token_rate: 0,
                out_token_rate: 0,
                deposit_near_amount: U128(1),
            },
        }
        .emit();

        PoolEvent::UpdatePool {
            pool: &ConversionPool {
                id: U64(1),
                creator: bob(),
                in_token: usdc(),
                in_token_balance: U128(1),
                out_token: usdt(),
                out_token_balance: U128(1),
                reversible: false,
                in_token_rate: 0,
                out_token_rate: 0,
                deposit_near_amount: U128(1),
            },
        }
        .emit();

        PoolEvent::DeletePool { pool_id: &U64(1) }.emit();

        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"data":{"pool":{"creator":"bob.near","deposit_near_amount":"1","id":"1","in_token":"usdc","in_token_balance":"1","in_token_rate":0,"out_token":"usdt","out_token_balance":"1","out_token_rate":0,"reversible":false}},"pool_event":"create_pool","standard":"convertor","version":"1.0.0"}"#
        );
        assert_eq!(
            test_utils::get_logs()[1],
            r#"EVENT_JSON:{"data":{"pool":{"creator":"bob.near","deposit_near_amount":"1","id":"1","in_token":"usdc","in_token_balance":"1","in_token_rate":0,"out_token":"usdt","out_token_balance":"1","out_token_rate":0,"reversible":false}},"pool_event":"update_pool","standard":"convertor","version":"1.0.0"}"#
        );
        assert_eq!(
            test_utils::get_logs()[2],
            r#"EVENT_JSON:{"data":{"pool_id":"1"},"pool_event":"delete_pool","standard":"convertor","version":"1.0.0"}"#
        );
    }
}
