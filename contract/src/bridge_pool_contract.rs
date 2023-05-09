use crate::detail;
use crate::{
    data::{self, BridgePool},
    error::Error,
    event::BridgePoolEvent,
};
use alloc::string::String;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractPackageHash, U256};
use contract_utils::{ContractContext, ContractStorage};

pub trait BridgePoolContract<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {
        BridgePool::init();
    }

    fn emit(&mut self, event: BridgePoolEvent) {
        data::emit(&event);
    }

    // outer function to get liquidity already in pool
    fn get_liquidity(&mut self, token_address: String) -> Result<U256, Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str())
                .map_err(|_| Error::NotContractPackageHash)?;

        let client_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let bridge_pool_instance = BridgePool::instance();
        bridge_pool_instance
            .get_liquidity_added_by_client(token_contract_package_hash, client_address)
    }

    // outer function to add liquidity to the pool
    fn add_liquidity(
        &mut self,
        amount: U256,
        token_address: String,
        bridge_pool_contract_package_hash_string: String,
    ) -> Result<(), Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str())
                .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_contract_package_hash = ContractPackageHash::from_formatted_str(
            bridge_pool_contract_package_hash_string.as_str(),
        )
        .map_err(|_| Error::NotBridgePoolContractPackageHash)?;

        let client_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let bridge_pool_instance = BridgePool::instance();
        bridge_pool_instance.add_liquidity(
            bridge_pool_contract_package_hash,
            token_contract_package_hash,
            client_address,
            amount,
        )?;

        self.emit(BridgePoolEvent::BridgeLiquidityAdded {
            actor: client_address,
            token: token_contract_package_hash,
            amount,
        });
        Ok(())
    }

    // outer function to remove liquidity from the pool
    fn remove_liquidity(&mut self, amount: U256, token_address: String) -> Result<(), Error> {
        let token_contract_package_hash =
            ContractPackageHash::from_formatted_str(token_address.as_str())
                .map_err(|_| Error::NotContractPackageHash)?;

        let client_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let bridge_pool_instance = BridgePool::instance();
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

    // outer function to swap liquidity
    fn swap(
        &mut self,
        token_address: String,
        amount: U256,
        target_network: U256,
        target_token: String,
    ) -> Result<(), Error> {
        let actor = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_instance = BridgePool::instance();
        bridge_pool_instance.swap(actor, token, target_token.clone(), amount, target_network)?;

        let target_address = target_token;

        self.emit(BridgePoolEvent::BridgeSwap {
            actor,
            token,
            target_network,
            target_address,
            amount,
        });
        Ok(())
    }

    // outer function to allow target
    fn allow_target(
        &mut self,
        token_address: String,
        token_name: String,
        target_network: U256,
        target_token: String,
    ) -> Result<(), Error> {
        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_instance = BridgePool::instance();
        bridge_pool_instance.allow_target(token, token_name, target_token, target_network)?;
        Ok(())
    }

    // outer function to withdraw liquidity from the pool securely
    fn withdraw_signed(
        &mut self,
        token_address: String,
        payee: String,
        amount: U256,
        chain_id: u64,
        salt: String,
        signature: String,
    ) -> Result<(), Error> {
        let actor = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let salt_array: [u8; 32] = hex::decode(salt)
            .map_err(|_| Error::SaltHexFail)?
            .try_into()
            .map_err(|_| Error::SaltWrongSize)?;
        let signature_vec = hex::decode(signature).unwrap();

        let bridge_pool_instance = BridgePool::instance();
        let signer = bridge_pool_instance.withdraw_signed(
            token,
            payee.clone(),
            amount,
            chain_id,
            salt_array,
            signature_vec,
            actor,
        )?;
        self.emit(BridgePoolEvent::TransferBySignature {
            signer,
            receiver: payee,
            token,
            amount,
        });
        Ok(())
    }

    // outer function to add signer
    fn add_signer(&mut self, signer: String) -> Result<(), Error> {
        let bridge_pool_instance = BridgePool::instance();
        if !is_lowercase(&signer) {
            Err(Error::SignerWrongFormat)
        } else {
            bridge_pool_instance.add_signer(signer);
            Ok(())
        }
    }

    // outer function to remove signer
    fn remove_signer(&mut self, signer: String) {
        let bridge_pool_instance = BridgePool::instance();
        bridge_pool_instance.remove_signer(signer)
    }

    // outer function to add signer
    fn check_signer(&mut self, signer: String) -> Result<bool, Error> {
        let bridge_pool_instance = BridgePool::instance();
        Ok(bridge_pool_instance.check_signer(signer)?)
    }
}

fn is_lowercase(s: &str) -> bool {
    for c in s.chars() {
        if !(c.is_lowercase() || c.is_ascii_digit()) {
            return false;
        }
    }
    true
}
