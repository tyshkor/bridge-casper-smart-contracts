use crate::{
    address::Address,
    data::{self, BrigdePool},
    error::Error,
    event::BridgePoolEvent,
};
use alloc::{
    fmt::format,
    string::{String, ToString},
};
use casper_types::{account::AccountHash, RuntimeArgs};
use casper_types::{runtime_args, ApiError, BlockTime, ContractPackageHash, Key, U256};
use contract_utils::{ContractContext, ContractStorage};
// use core::convert::TryInto;
use crate::detail;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::ContractHash;

pub trait BridgePoolContract<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {
        BrigdePool::init();
    }

    fn name(&self) -> String {
        data::name()
    }

    fn address(&self) -> String {
        data::address()
    }

    fn emit(&mut self, event: BridgePoolEvent) {
        data::emit(&event);
    }

    fn get_liquidity(&mut self, token_address: String) -> Result<U256, Error> {
        let client_address = detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);
        let client = alloc::format!("{:#?}", client_address);

        let bridge_pool_instance = BrigdePool::instance();
        let res = bridge_pool_instance.get_liquidity_added_by_client(token_address, client).unwrap_or_revert_with(Error::NegativeReward);

        Ok(res)
    }

    fn add_liquidity(&mut self, amount: U256, token_address: String) -> Result<(), Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str()).unwrap();

        let client_address = detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);
        let client = alloc::format!("{:#?}", client_address);

        let bridge_pool_instance = BrigdePool::instance();
        // bridge_pool_instance.add_liquidity(token_address, client, amount);

        // self.emit(BridgePoolEvent::BridgeLiquidityAdded {
        //     actor: client_address,
        //     token: token_contract_package_hash,
        //     amount,
        // });
        Ok(())
    }

    fn remove_liquidity(&mut self, amount: U256, token_address: String) -> Result<(), Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str()).unwrap();
        
        let client_address = detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);
        let client = alloc::format!("{:#?}", client_address);

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.remove_liquidity(token_address, client, amount);

        self.emit(BridgePoolEvent::BridgeLiquidityRemoved {
            actor: client_address,
            token: token_contract_package_hash,
            amount,
        });
        Ok(())
    }

    fn swap(&mut self, amount: U256, token_address: String) -> Result<(), Error> {
        // self.emit(BridgePoolEvent::BridgeSwap { actor: (), token: (), target_network: (), target_token: (), target_address: (), amount: U256::from(0i64) });
        Ok(())
    }
}
