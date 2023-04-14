// #![no_std]
// #![no_main]

// #[cfg(not(target_arch = "wasm32"))]
// // compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// extern crate alloc;
// // use crate::alloc::string::ToString;
// use alloc::string::String;

// use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

// use casper_erc20::{
//     constants::{
//         ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
//         NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
//         SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
//     },
//     Address, ERC20,
// };
// use casper_types::U256;
// // use casper_types::u8;
// use casper_types::CLValue;
/*


casper-client put-deploy \
--chain-name casper-test \
--node-address http://44.208.234.65:7777 \
--secret-key ./keys/secret_key.pem \
--session-path ./target/wasm32-unknown-unknown/release/erc20_token.wasm \
--payment-amount 200000000000 \
--session-arg "name:string='FERRUM_ERC20'" \
--session-arg "symbol:string='F_ERC20'" \
--session-arg "decimals:u8='8'" \
--session-arg "total_supply:U256='500000'"



*/

/*
        casper-client put-deploy \
        --chain-name casper-test \
        --node-address http://159.65.118.250:7777 \
        --secret-key ./keys/secret_key.pem \
        --payment-amount 4000000000 \
        --session-hash 7d4d39dc4d3c017e3153c221279dd5343b7eada42b97e8a554b9bc0d7cf4f602 \
        --session-entry-point "mint" 
        
*/

/*

        casper-client put-deploy \
        --chain-name casper-test \
        --node-address http://159.65.118.250:7777 \
        --secret-key ./keys/secret_key.pem \
        --payment-amount 4000000000 \
        --session-hash 7d4d39dc4d3c017e3153c221279dd5343b7eada42b97e8a554b9bc0d7cf4f602 \
        --session-entry-point "approve" \
        --session-arg "owner:public_key='017fbbccf39a639a1a5f469e3fb210d9f355b532bd786f945409f0fc9a8c6313b1'" \
        --session-arg "spender:key='f281c7a263745a4f8c05638af8e92924d7ea50de929621b017a4114d8e5dda52'"  \
        --session-arg "amount:u256='10'"
*/


// #[no_mangle]
// fn call() {
//     let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
//     let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
//     let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);
//     let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
//     // let total_supply = U256::from(totalsupply);
//         // let decimal: u8 = 8;
//     // let decimals = U256::from(decimal);
//     // let name: String = "FerrumToken".to_string();
//     // let symbol: String = "FRMT".to_string();
//     // let total_supply: u8 = 1_00;
//     // let decimal: u8 = 8;
//     // let decimals = U256::from(decimal);
//     let _token = ERC20::install(name, symbol, total_supply, decimals).unwrap_or_revert();
// }

// #[no_mangle]
// pub extern "C" fn name(){
//     let name = ERC20::default().name();
//     runtime::ret(CLValue::from_t(name).unwrap_or_revert());
// }

// #[no_mangle]
// pub extern "C" fn symbol(){
//     let symbol = ERC20::default().symbol();
//     runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
// }

// #[no_mangle]
// pub extern "C" fn decimals(){
//     let decimals = ERC20::default().decimals();
//     runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
// }

// #[no_mangle]
// pub extern "C" fn total_supply(){
//     let total_supply = ERC20::default().total_supply();
//     runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
// }

// #[no_mangle]
// pub extern "C" fn balance_of(){
//     let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
//     let balance = ERC20::default().balance_of(address);
//     runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
// }

// #[no_mangle]
// pub extern "C" fn transfer(){
//     let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
//     let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

//     ERC20::default().transfer(recipient, amount).unwrap_or_revert();
// }

// #[no_mangle]
// pub extern "C" fn transfer_from(){
//     let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
//     let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
//     let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    
//     ERC20::default().transfer_from(owner, recipient, amount).unwrap_or_revert();
// }

// #[no_mangle]
// pub extern "C" fn approve(){
//     let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
//     let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

//     ERC20::default().approve(spender, amount).unwrap_or_revert();
// }

// #[no_mangle]
// pub extern "C" fn allowance(){
//     let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
//     let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
//     let amount = ERC20::default().allowance(owner, spender);
//     runtime::ret(CLValue::from_t(amount).unwrap_or_revert());
// }

// #[no_mangle]
// pub extern "C" fn mint(){
//     let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
//     let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    
//     ERC20::default().mint(owner, amount).unwrap_or_revert();
// }

// #[no_mangle]
// pub extern "C" fn burn(){
//     let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
//     let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

//     ERC20::default().burn(owner, amount).unwrap_or_revert();
// }