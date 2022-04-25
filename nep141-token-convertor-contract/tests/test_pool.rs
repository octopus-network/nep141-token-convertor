use near_sdk::AccountId;
pub mod common;

fn test_create_pool() {
    let root = init_simulator(None);
    let user = root.create_user("user".to_string(), to_yocto("100"));
}
