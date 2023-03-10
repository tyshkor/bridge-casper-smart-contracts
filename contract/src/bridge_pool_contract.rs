use crate::address::Address;
use crate::detail;
use crate::{
    data::{self, BrigdePool},
    error::Error,
    event::BridgePoolEvent,
};
use alloc::string::String;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::account::AccountHash;
use casper_types::{ContractPackageHash, U256};
use contract_utils::{ContractContext, ContractStorage};

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
        let _ = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let client_address =
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.get_liquidity_added_by_client(token_address, client_address)
    }

    fn add_liquidity(&mut self, amount: U256, token_address: String) -> Result<(), Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str())
                .map_err(|_| Error::NotContractPackageHash)?;

        let client_address =
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.add_liquidity(token_contract_package_hash, client_address, amount)?;

        self.emit(BridgePoolEvent::BridgeLiquidityAdded {
            actor: client_address,
            token: token_contract_package_hash,
            amount,
        });
        Ok(())
    }

    fn remove_liquidity(&mut self, amount: U256, token_address: String) -> Result<(), Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str())
                .map_err(|_| Error::NotContractPackageHash)?;

        let client_address =
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.remove_liquidity(
            token_contract_package_hash,
            client_address,
            amount,
        )?;

        self.emit(BridgePoolEvent::BridgeLiquidityRemoved {
            actor: client_address,
            token: token_contract_package_hash,
            amount,
        });
        Ok(())
    }

    fn swap(
        &mut self,
        token_address: String,
        amount: U256,
        target_network: U256,
        target_token: String,
    ) -> Result<(), Error> {
        let actor =
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);

        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.swap(actor, token, target_token.clone(), amount, target_network)?;

        self.emit(BridgePoolEvent::BridgeSwap {
            actor,
            token,
            target_network,
            target_token,
            target_address: actor,
            amount,
        });
        Ok(())
    }

    fn allow_target(
        &mut self,
        token_address: String,
        target_network: U256,
        target_token: String,
    ) -> Result<(), Error> {
        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.allow_target(token, target_token.clone(), target_network)?;
        Ok(())
    }

    fn withdraw_signed(
        &mut self,
        token_address: String,
        payee: String,
        amount: U256,
        salt: [u8; 32],
        signature: alloc::vec::Vec<u8>,
    ) -> Result<(), Error> {
        let actor =
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);

        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let payee_account_hash: AccountHash =
            AccountHash::from_formatted_str(&payee).map_err(|_| Error::CasperAccountHashParsing)?;
        let payee_address: Address = From::from(payee_account_hash);

        let bridge_pool_instance = BrigdePool::instance();
        bridge_pool_instance.withdraw_signed(token, payee_address, amount, salt, signature)?;
        self.emit(BridgePoolEvent::TransferBySignature {
            signer: actor,
            reciever: payee,
            token,
            amount,
        });
        Ok(())
    }
}
