// General constants
pub const CONTRACT_VERSION_KEY: &str = "version";
pub const CONTRACT_KEY: &str = "bridge_pool";
pub const BRIDGE_POOL_CONTRACT_PACKAGE_HASH: &str = "bridge_pool_contract_package_hash";
pub const ADMIN_ADDRESS: &str = "admin_address";
pub const BRIDGE_POOL_CONTRACT_HASH: &str = "bridge_pool_contract_hash";
pub const BRIDGE_POOL_PACKAGE_NAME: &str = "bridge_pool_package_name";
pub const BRIDGE_POOL_ACCESS_UREF: &str = "bridge_pool_access_uref";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

// Group constants
pub const CONSTRUCTOR_GROUP: &str = "constructor_group";
pub const ADMIN_GROUP: &str = "admin_group";
pub const ADMIN_ACCESS_UREF: &str = "admin_access_uref";

// Bridge pool entry point constants
pub const ENTRY_POINT_GET_LIQUIDITY: &str = "get_liquidity";
pub const ENTRY_POINT_ADD_LIQUIDITY: &str = "add_liquidity";
pub const ENTRY_POINT_ADMIN_ADD_LIQUIDITY: &str = "admin_add_liquidity";
pub const ENTRY_POINT_REMOVE_LIQUIDITY: &str = "remove_liquidity";
pub const ENTRY_POINT_SWAP: &str = "swap";
pub const ENTRY_POINT_SWAP_REVERSE: &str = "swap_reverse";
pub const ENTRY_POINT_ALLOW_TARGET: &str = "allow_target";
pub const ENTRY_POINT_WITHDRAW_SIGNED: &str = "withdraw_signed";
pub const ENTRY_POINT_WITHDRAW: &str = "withdraw";
pub const ENTRY_POINT_ADD_SIGNER: &str = "add_signer";
pub const ENTRY_POINT_REMOVE_SIGNER: &str = "remove_signer";
pub const ENTRY_POINT_CONSTRUCTOR: &str = "constructor";
pub const ENTRY_POINT_CHECK_SIGNER: &str = "check_signer";

// ERC20 entry point constants
pub const ERC20_ENTRY_POINT_TRANSFER: &str = "transfer";
pub const ERC20_ENTRY_POINT_TRANSFER_FROM: &str = "transfer_from";

// Agruments constants
pub const AMOUNT: &str = "amount";
pub const SIGNER: &str = "signer";
pub const TOKEN: &str = "token";
pub const TOKEN_ADDRESS: &str = "token_address";
pub const TARGET_TOKEN: &str = "target_token";
pub const TARGET_ADDRESS: &str = "target_address";
pub const TARGET_NETWORK: &str = "target_network";
pub const TOKEN_NAME: &str = "token_name";
pub const PAYEE: &str = "payee";
pub const RECEIVER: &str = "receiver";
pub const RECIPIENT: &str = "recipient";
pub const OWNER: &str = "owner";
pub const SALT: &str = "salt";
pub const SIGNATURE: &str = "signature";
pub const CHAIN_ID: &str = "chain_id";
pub const TOKEN_RECIPIENT: &str = "token_recipient";
pub const CALLER: &str = "caller";
pub const ACTOR: &str = "actor";

// Dictionary name constants
pub const ACCOUNT_HASH_LIQUIDITIES_DICT: &str = "account_hash_liquidities_dict";
pub const HASH_ADDR_LIQUIDITIES_DICT: &str = "hash_addr_liquidities_dict";
pub const ADMIN_LIQUIDITIES_DICT: &str = "admin_liquidities_dict";
pub const ALLOWED_TARGETS_DICT: &str = "allowed_targets_dict";
pub const USED_HASHES_DICT: &str = "used_hashes_dict";
pub const SIGNERS_DICT: &str = "signers_dict";
pub const TOKEN_CONTRACT_PACKAGE_HASH_DICT_NAME: &str = "token_contract_package_hash_dict_name";

// Event constants
pub const EVENT_TYPE: &str = "event_type";
pub const EVENT_BRIDGE_LIQUIDITY_ADDED: &str = "bridge_liquidity_added";
pub const EVENT_BRIDGE_LIQUIDITY_ADDED_BY_ADMIN: &str = "bridge_liquidity_added_by_admin";
pub const EVENT_BRIDGE_LIQUIDITY_REMOVED: &str = "bridge_liquidity_removed";
pub const EVENT_BRIDGE_SWAP: &str = "bridge_swap";
pub const EVENT_BRIDGE_SWAP_TO: &str = "bridge_swap_to";
pub const EVENT_BRIDGE_TRANSFER_BY_SIGNATURE: &str = "bridge_transfer_by_signature";
