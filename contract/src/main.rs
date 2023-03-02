#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec
};
use bridge_pool::bridge_pool_contract::BridgePoolContract;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, URef, U256, CLTyped, Parameter,
};
use contract_utils::{ContractContext, OnChainContractStorage};

const ENTRY_POINT_GET_LIQUIDITY: &str = "get_liquidity";
const ENTRY_POINT_ADD_LIQUIDITY: &str = "add_liquidity";
const ENTRY_POINT_REMOVE_LIQUIDITY: &str = "remove_liquidity";
const ENTRY_POINT_SWAP: &str = "swap";

const CONTRACT_VERSION_KEY: &str = "version";
const LIQUIDITY_KEY: &str = "liquidity";
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
    Contract::default().constructor();
}

#[no_mangle]
pub extern "C" fn get_liquidity() {
    let token_address = runtime::get_named_arg::<String>("token");
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
    let token_address = runtime::get_named_arg::<String>("token");
    let ret = Contract::default()
        .swap(amount, token_address)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    // Initialize the count to 0 locally
    let count_start = storage::new_uref(0_i32);

    // In the named keys of the contract, add a key for the count
    let mut counter_named_keys = NamedKeys::new();
    let key_name = String::from(LIQUIDITY_KEY);
    counter_named_keys.insert(key_name, count_start.into());

    // Create entry points for this contract
    let mut bridge_pool_entry_points = EntryPoints::new();

    bridge_pool_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_GET_LIQUIDITY,
        vec![
            Parameter::new("token_address", String::cl_type()),
        ],
        CLType::I32,
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
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create a new contract package that can be upgraded
    let (stored_contract_hash, contract_version) = storage::new_contract(
        bridge_pool_entry_points,
        Some(counter_named_keys),
        Some("bridge_pool_package_name".to_string()),
        Some("bridge_pool_access_uref".to_string()),
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
