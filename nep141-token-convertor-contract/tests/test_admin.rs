// pub use crate::common::constant::string_to_account;
// pub use crate::common::contracts::{deploy_convertor_contract, setup_convertor_contract};
// pub use common::convertor::Convertor;
// use near_sdk::json_types::U128;
// pub use near_sdk::AccountId;
// pub use near_sdk_sim::{call, deploy, init_simulator, to_yocto, view, ContractAccount};
// use nep141_token_convertor_contract::FtMetaData;
//
// mod common;
//
// #[test]
// fn test_whitelist() {
//     let (root, owner, convertor) = setup_convertor_contract();
//     let mut tokens = vec![
//         FtMetaData {
//             token_id: string_to_account("usdt"),
//             decimals: 6,
//         },
//         FtMetaData {
//             token_id: string_to_account("usdc"),
//             decimals: 6,
//         },
//         FtMetaData {
//             token_id: string_to_account("usdn"),
//             decimals: 8,
//         },
//     ];
//     convertor
//         .extend_whitelisted_tokens(&owner, tokens.clone())
//         .assert_success();
//     assert_eq!(
//         convertor.get_whitelist(),
//         tokens,
//         "extend whitelist not right "
//     );
//
//     tokens.pop();
//     tokens.pop();
//     let remove_token_ids = vec![string_to_account("usdn"), string_to_account("usdc")];
//     convertor.remove_whitelisted_tokens(&owner, remove_token_ids.clone());
//     assert_eq!(convertor.get_whitelist(), tokens, "remove token not right");
//     assert!(
//         !convertor
//             .extend_whitelisted_tokens(&root, tokens.clone())
//             .is_ok(),
//         "should failed by owner access check"
//     );
//     assert!(
//         !convertor
//             .remove_whitelisted_tokens(&root, remove_token_ids.clone())
//             .is_ok(),
//         "should failed by owner access check"
//     )
// }
//
// #[test]
// fn test_set_pool_create_deposit_amount() {
//     let (root, owner, convertor) = setup_convertor_contract();
//     assert!(
//         !convertor
//             .set_pool_create_deposit_amount(&root, U128::from(1))
//             .is_ok(),
//         "should failed by owner access check"
//     );
//     convertor
//         .set_pool_create_deposit_amount(&owner, U128::from(1))
//         .assert_success();
// }
