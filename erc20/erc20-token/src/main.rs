#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");



extern crate alloc;
use core::panic::PanicInfo;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
    },
    Address, ERC20, Error,
};
use casper_types::{U256, system::CallStackElement};
// use casper_types::u8;
use casper_types::CLValue;

#[no_mangle]
pub extern "C" fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    // let total_supply = U256::from(totalsupply);
        // let decimal: u8 = 8;
    // let decimals = U256::from(decimal);
    // let name: String = "FerrumToken".to_string();
    // let symbol: String = "FRMT".to_string();
    // let total_supply: u8 = 1_00;
    // let decimal: u8 = 8;
    // let decimals = U256::from(decimal);
    let _token = ERC20::install(name, symbol, decimals, total_supply).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn name(){
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol(){
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals(){
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply(){
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of(){
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer(){
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().transfer(recipient, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn transfer_from(){
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    
    ERC20::default().transfer_from(owner, recipient, amount).unwrap_or_revert_with(Error::User(9000u16));
}

#[no_mangle]
pub extern "C" fn approve(){
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance(){
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(amount).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn burn(){
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().burn(owner, amount).unwrap_or_revert();
}

/// Gets the immediate call stack element of the current execution.
fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

pub(crate) fn get_immediate_caller_address() -> Result<Address, Error> {
    get_immediate_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

#[no_mangle]
pub extern "C" fn mint(){
    // let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    // let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    let owner: Address = get_immediate_caller_address().unwrap();

    let amount: U256 = U256::from(10000i64);
    
    ERC20::default().mint(owner, amount).unwrap_or_revert();
}
