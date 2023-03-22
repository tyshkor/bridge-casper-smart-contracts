#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use crate::runtime_args::RuntimeArgs;
use alloc::{
    string::{String, ToString},
    vec,
};
use bridge_pool::bridge_pool_contract::BridgePoolContract;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, Key, Parameter, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};

const ENTRY_POINT_GET_LIQUIDITY: &str = "get_liquidity";
const ENTRY_POINT_ADD_LIQUIDITY: &str = "add_liquidity";
const ENTRY_POINT_REMOVE_LIQUIDITY: &str = "remove_liquidity";
const ENTRY_POINT_SWAP: &str = "swap";
const ENTRY_POINT_ALLOW_TARGET: &str = "allow_target";
const ENTRY_POINT_WITHDRAW_SIGNED: &str = "withdraw_signed";

const CONTRACT_VERSION_KEY: &str = "version";
const CONTRACT_KEY: &str = "bridge_pool";

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
        runtime::get_named_arg::<Key>("bridge_pool_contract_package_hash");

    runtime::put_key(
        "bridge_pool_contract_package_hash",
        bridge_pool_contract_package_hash.into(),
    );

    Contract::default().constructor();
}

#[no_mangle]
pub extern "C" fn get_liquidity() {
    let token_address = runtime::get_named_arg::<String>("token_address");
    let result = Contract::default()
        .get_liquidity(token_address)
        .unwrap_or_revert();

    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn add_liquidity() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let token_address = runtime::get_named_arg::<String>("token_address");
    let ret = Contract::default()
        .add_liquidity(amount, token_address)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn remove_liquidity() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let token_address = runtime::get_named_arg::<String>("token_address");
    let ret = Contract::default()
        .remove_liquidity(amount, token_address)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn swap() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let token_address = runtime::get_named_arg::<String>("token_address");
    let target_network = runtime::get_named_arg::<U256>("target_network");
    let target_token = runtime::get_named_arg::<String>("target_token");
    let ret = Contract::default()
        .swap(token_address, amount, target_network, target_token)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn allow_target() {
    let token_address = runtime::get_named_arg::<String>("token_address");
    let token_name = runtime::get_named_arg::<String>("token_name");
    let target_network = runtime::get_named_arg::<U256>("target_network");
    let target_token = runtime::get_named_arg::<String>("target_token");
    let ret = Contract::default()
        .allow_target(token_address, token_name, target_network, target_token)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn withdraw_signed() {
    let token_address = runtime::get_named_arg::<String>("token_address");
    let payee = runtime::get_named_arg::<String>("payee");
    let amount = runtime::get_named_arg::<U256>("amount");
    let salt = runtime::get_named_arg::<[u8; 32]>("salt");
    let signature = runtime::get_named_arg::<alloc::vec::Vec<u8>>("signature");
    let ret = Contract::default()
        .withdraw_signed(token_address, payee, amount, salt, signature)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let bridge_pool_named_keys = NamedKeys::new();

    // Create entry points for this contract
    let mut bridge_pool_entry_points = EntryPoints::new();

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_GET_LIQUIDITY,
        vec![Parameter::new("token_address", String::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ADD_LIQUIDITY,
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("token_address", String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_REMOVE_LIQUIDITY,
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("token_address", String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_SWAP,
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("token_address", String::cl_type()),
            Parameter::new("target_network", U256::cl_type()),
            Parameter::new("target_token", String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ALLOW_TARGET,
        vec![
            Parameter::new("token_address", String::cl_type()),
            Parameter::new("token_name", String::cl_type()),
            Parameter::new("target_network", U256::cl_type()),
            Parameter::new("target_token", String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHDRAW_SIGNED,
        vec![
            Parameter::new("token_address", String::cl_type()),
            Parameter::new("payee", String::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("salt", <[u8; 32]>::cl_type()),
            Parameter::new("signature", <alloc::vec::Vec<u8>>::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create a new contract package that can be upgraded
    let (stored_contract_hash, contract_version) = storage::new_contract(
        bridge_pool_entry_points,
        Some(bridge_pool_named_keys),
        Some("bridge_pool_package_name".to_string()),
        Some("bridge_pool_access_uref".to_string()),
    );

    let package_hash_key: Key = stored_contract_hash.into();

    let _: () = runtime::call_contract(
        stored_contract_hash,
        "constructor",
        runtime_args! {
            "bridge_pool_contract_package_hash" => package_hash_key,
        },
    );

    /* To create a locked contract instead, use new_locked_contract and throw away the contract version returned
    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None); */

    // Store the contract version in the context's named keys
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}
