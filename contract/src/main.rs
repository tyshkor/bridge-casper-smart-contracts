#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    collections::BTreeSet,
    string::{String, ToString},
    vec,
};
use bridge_pool::{
    bridge_pool_contract::BridgePoolContract,
    consts::{
        ADMIN_ACCESS_UREF, ADMIN_GROUP, AMOUNT, BRIDGE_POOL_ACCESS_UREF, BRIDGE_POOL_CONTRACT_HASH,
        BRIDGE_POOL_CONTRACT_PACKAGE_HASH, BRIDGE_POOL_PACKAGE_NAME, CALLER, CHAIN_ID,
        CONSTRUCTOR_GROUP, CONTRACT_KEY, CONTRACT_VERSION_KEY, ENTRY_POINT_ADD_LIQUIDITY,
        ENTRY_POINT_ADD_SIGNER, ENTRY_POINT_ALLOW_TARGET, ENTRY_POINT_CHECK_SIGNER,
        ENTRY_POINT_CONSTRUCTOR, ENTRY_POINT_GET_LIQUIDITY, ENTRY_POINT_REMOVE_LIQUIDITY,
        ENTRY_POINT_REMOVE_SIGNER, ENTRY_POINT_SWAP, ENTRY_POINT_WITHDRAW_SIGNED, PAYEE, SALT,
        SIGNATURE, SIGNER, TARGET_ADDRESS, TARGET_NETWORK, TARGET_TOKEN, TOKEN_ADDRESS, TOKEN_NAME,
        TOKEN_RECIPIENT,
    },
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::RuntimeArgs;
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, Parameter, U256,
};
use casper_types::{Group, Key, URef};
use contract_utils::{ContractContext, OnChainContractStorage};

#[derive(Default)]
struct Contract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Contract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl BridgePoolContract<OnChainContractStorage> for Contract {}

impl Contract {
    fn constructor(&mut self) {
        BridgePoolContract::init(self);
    }
}

#[no_mangle]
pub extern "C" fn constructor() {
    let bridge_pool_contract_package_hash =
        runtime::get_named_arg::<Key>(BRIDGE_POOL_CONTRACT_PACKAGE_HASH);
    runtime::put_key(
        BRIDGE_POOL_CONTRACT_PACKAGE_HASH,
        bridge_pool_contract_package_hash,
    );

    Contract::default().constructor();
}

#[no_mangle]
pub extern "C" fn get_liquidity() {
    let token_address = runtime::get_named_arg::<String>(TOKEN_ADDRESS);
    let result = Contract::default()
        .get_liquidity(token_address)
        .unwrap_or_revert();

    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn add_liquidity() {
    let amount = runtime::get_named_arg::<U256>(AMOUNT);
    let token_address = runtime::get_named_arg::<String>(TOKEN_ADDRESS);
    let bridge_pool_contract_package_hash =
        runtime::get_named_arg::<String>(BRIDGE_POOL_CONTRACT_PACKAGE_HASH);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default()
        .add_liquidity(amount, token_address, bridge_pool_contract_package_hash)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn remove_liquidity() {
    let amount = runtime::get_named_arg::<U256>(AMOUNT);
    let token_address = runtime::get_named_arg::<String>(TOKEN_ADDRESS);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default()
        .remove_liquidity(amount, token_address)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn swap() {
    let amount = runtime::get_named_arg::<U256>(AMOUNT);
    let token_address = runtime::get_named_arg::<String>(TOKEN_ADDRESS);
    let target_network = runtime::get_named_arg::<U256>(TARGET_NETWORK);
    let target_token = runtime::get_named_arg::<String>(TARGET_TOKEN);
    let target_address = runtime::get_named_arg::<String>(TARGET_ADDRESS);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default()
        .swap(
            token_address,
            amount,
            target_network,
            target_token,
            target_address,
        )
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn allow_target() {
    let token_address = runtime::get_named_arg::<String>(TOKEN_ADDRESS);
    let token_name = runtime::get_named_arg::<String>(TOKEN_NAME);
    let target_network = runtime::get_named_arg::<U256>(TARGET_NETWORK);
    let target_token = runtime::get_named_arg::<String>(TARGET_TOKEN);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default()
        .allow_target(token_address, token_name, target_network, target_token)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn withdraw_signed() {
    let token_address = runtime::get_named_arg::<String>(TOKEN_ADDRESS);
    let payee = runtime::get_named_arg::<String>(PAYEE);
    let amount = runtime::get_named_arg::<U256>(AMOUNT);
    let chain_id = runtime::get_named_arg::<u64>(CHAIN_ID);
    let salt = runtime::get_named_arg::<String>(SALT);
    let signature = runtime::get_named_arg::<String>(SIGNATURE);
    let token_recipient = runtime::get_named_arg::<String>(TOKEN_RECIPIENT);
    let caller = runtime::get_named_arg::<String>(CALLER);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default()
        .withdraw_signed(
            token_address,
            payee,
            amount,
            chain_id,
            salt,
            token_recipient,
            signature,
            caller,
        )
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn add_signer() {
    let signer = runtime::get_named_arg::<String>(SIGNER);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default().add_signer(signer).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn check_signer() {
    let signer = runtime::get_named_arg::<String>(SIGNER);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default().check_signer(signer).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn remove_signer() {
    let signer = runtime::get_named_arg::<String>(SIGNER);
    #[allow(clippy::let_unit_value)]
    let ret = Contract::default().remove_signer(signer);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let bridge_pool_named_keys = NamedKeys::new();

    // Create entry points for this contract
    let mut bridge_pool_entry_points = EntryPoints::new();

    let admin_group = Group::new(ADMIN_GROUP);

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_CONSTRUCTOR,
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new(CONSTRUCTOR_GROUP)]),
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_GET_LIQUIDITY,
        vec![Parameter::new(TOKEN_ADDRESS, String::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ADD_LIQUIDITY,
        vec![
            Parameter::new(AMOUNT, U256::cl_type()),
            Parameter::new(TOKEN_ADDRESS, String::cl_type()),
            Parameter::new(BRIDGE_POOL_CONTRACT_PACKAGE_HASH, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_REMOVE_LIQUIDITY,
        vec![
            Parameter::new(AMOUNT, U256::cl_type()),
            Parameter::new(TOKEN_ADDRESS, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_SWAP,
        vec![
            Parameter::new(AMOUNT, U256::cl_type()),
            Parameter::new(TOKEN_ADDRESS, String::cl_type()),
            Parameter::new(TARGET_NETWORK, U256::cl_type()),
            Parameter::new(TARGET_TOKEN, String::cl_type()),
            Parameter::new(TARGET_ADDRESS, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ALLOW_TARGET,
        vec![
            Parameter::new(TOKEN_ADDRESS, String::cl_type()),
            Parameter::new(TOKEN_NAME, String::cl_type()),
            Parameter::new(TARGET_NETWORK, U256::cl_type()),
            Parameter::new(TARGET_TOKEN, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![admin_group.clone()]),
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHDRAW_SIGNED,
        vec![
            Parameter::new(PAYEE, String::cl_type()),
            Parameter::new(CHAIN_ID, u64::cl_type()),
            Parameter::new(TOKEN_ADDRESS, String::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
            Parameter::new(SALT, String::cl_type()),
            Parameter::new(SIGNATURE, String::cl_type()),
            Parameter::new(TOKEN_RECIPIENT, String::cl_type()),
            Parameter::new(CALLER, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ADD_SIGNER,
        vec![Parameter::new(SIGNER, String::cl_type())],
        CLType::Unit,
        EntryPointAccess::Groups(vec![admin_group.clone()]),
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_REMOVE_SIGNER,
        vec![Parameter::new(SIGNER, String::cl_type())],
        CLType::Unit,
        EntryPointAccess::Groups(vec![admin_group.clone()]),
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_CHECK_SIGNER,
        vec![Parameter::new(SIGNER, String::cl_type())],
        CLType::Bool,
        EntryPointAccess::Groups(vec![admin_group]),
        EntryPointType::Contract,
    ));

    // Create a new contract package that can be upgraded
    let (stored_contract_hash, contract_version) = storage::new_contract(
        bridge_pool_entry_points,
        Some(bridge_pool_named_keys),
        Some(BRIDGE_POOL_PACKAGE_NAME.to_string()),
        Some(BRIDGE_POOL_ACCESS_UREF.to_string()),
    );

    runtime::put_key(BRIDGE_POOL_CONTRACT_HASH, stored_contract_hash.into());

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(BRIDGE_POOL_PACKAGE_NAME)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let package_hash_key: Key = package_hash.into();

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, CONSTRUCTOR_GROUP, 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(
        stored_contract_hash,
        ENTRY_POINT_CONSTRUCTOR,
        runtime_args! {
            BRIDGE_POOL_CONTRACT_PACKAGE_HASH => package_hash_key,
        },
    );

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, CONSTRUCTOR_GROUP, urefs)
        .unwrap_or_revert();

    let mut admin_group =
        storage::create_contract_user_group(package_hash, ADMIN_GROUP, 1, Default::default())
            .unwrap();
    runtime::put_key(ADMIN_ACCESS_UREF, admin_group.pop().unwrap().into());

    runtime::put_key(BRIDGE_POOL_CONTRACT_PACKAGE_HASH, package_hash_key);

    /* To create a locked contract instead, use new_locked_contract and throw away the contract version returned
    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None); */

    // Store the contract version in the context's named keys
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}
