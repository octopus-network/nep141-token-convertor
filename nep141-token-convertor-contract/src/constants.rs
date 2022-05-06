use near_sdk::StorageUsage;

pub const T_GAS_FOR_FT_TRANSFER: u64 = 10;
pub const T_GAS_FOR_RESOLVE_TRANSFER: u64 = 20;
// pub const GAS_FOR_FT_TRANSFER_CALL: u64 = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;

pub const U128_STORAGE: StorageUsage = 16;
// const U64_STORAGE: StorageUsage = 8;
pub const U32_STORAGE: StorageUsage = 4;
/// max length of account id is 64 bytes. We charge per byte.
pub const ACC_ID_STORAGE: StorageUsage = 64;
/// As a key, 4 bytes length would be added to the head
pub const ACC_ID_AS_KEY_STORAGE: StorageUsage = ACC_ID_STORAGE + 4;
/// As a near_sdk::collection key, 1 byte for prefix
pub const ACC_ID_AS_CLT_KEY_STORAGE: StorageUsage = ACC_ID_AS_KEY_STORAGE + 1;

/// ACC_ID: the Contract accounts map key length
/// + VAccount enum: 1 byte
/// + U128_STORAGE: near_amount_for_storage storage
/// + U32_STORAGE: tokens HashMap length
pub const INIT_ACCOUNT_STORAGE: StorageUsage =
    ACC_ID_AS_CLT_KEY_STORAGE + 1 + U32_STORAGE + U128_STORAGE;

/// Defining PREPAY_STORAGE is the maximum StorageUsage that can be occupied after any contract interfaces executing
/// now the maximum StorageUsage is delete_pool:
/// -if user has registered, it may add 2 entry into HashMap<AccountId, Balance>.
/// -if user hasn't registered, it will add INIT_ACCOUNT_STORAGE.
pub const PREPAY_STORAGE_FOR_REGISTERED: StorageUsage = 2 * (ACC_ID_AS_KEY_STORAGE + U128_STORAGE);

/// if user haven't registered, should add INIT_ACCOUNT_STORAGE.
pub const PREPAY_STORAGE_FOR_UNREGISTERED: StorageUsage =
    INIT_ACCOUNT_STORAGE + PREPAY_STORAGE_FOR_REGISTERED;
