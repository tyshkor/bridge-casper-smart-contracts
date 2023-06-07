use crate::detail;
use crate::{
    data::{self, BridgePool},
    error::Error,
    event::BridgePoolEvent,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::RuntimeArgs;
use casper_types::{runtime_args, ContractPackageHash, U256};
use contract_utils::keccak::{keccak256, keccak256_hash};
use contract_utils::{ContractContext, ContractStorage};
use k256::ecdsa::{
    recoverable::Signature as RecoverableSignature, signature::Signature as NonRecoverableSignature,
};
use secp256k1::{Message, Secp256k1};

const AMOUNT: &str = "amount";

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
        target_address: String,
    ) -> Result<(), Error> {
        let actor = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_instance = BridgePool::instance();
        bridge_pool_instance.swap(actor, token, target_token, amount, target_network)?;

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
    #[allow(clippy::too_many_arguments)]
    fn withdraw_signed(
        &mut self,
        token_address: String,
        payee: String,
        amount: U256,
        chain_id: u64,
        salt: String,
        receiver: String,
        signature: String,
    ) -> Result<(), Error> {
        let actor = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerFail);

        let token = ContractPackageHash::from_formatted_str(token_address.as_str())
            .map_err(|_| Error::NotContractPackageHash)?;

        let bridge_pool_instance = BridgePool::instance();

        let signature = hex::decode(signature).unwrap();

        let salt: [u8; 32] = hex::decode(salt)
            .map_err(|_| Error::SaltHexFail)?
            .try_into()
            .map_err(|_| Error::SaltWrongSize)?;

        let message_hash = hex::encode(keccak256(
            hex::encode(keccak256(
                &[
                    token.to_formatted_string().as_bytes(),
                    payee.as_bytes(),
                    amount.to_string().as_bytes(),
                    receiver.as_bytes(),
                    &chain_id.to_be_bytes(),
                    &salt,
                ]
                .concat()[..],
            ))
            .as_bytes(),
        ));

        let signature_rec = if signature.len() == 65 {
            RecoverableSignature::from_bytes(&signature[..])
                .map_err(|_| Error::RecoverableSignatureTryFromFail)?
        } else {
            NonRecoverableSignature::from_bytes(&signature[..])
                .map_err(|_| Error::NonRecoverableSignatureTryFromFail)?
        };

        let hash =
            &hex::decode(message_hash.clone()).map_err(|_| Error::MessageHashHexDecodingFail)?[..];
        let sig = &signature_rec;

        let s = Secp256k1::new();
        let msg = Message::from_slice(hash).unwrap();
        let mut sig_compact: Vec<u8> = sig.r().to_bytes().to_vec();
        sig_compact.extend(&sig.s().to_bytes().to_vec());
        let id_u8: u8 = From::from(sig.recovery_id());
        let sig_v = secp256k1::ecdsa::RecoveryId::from_i32(id_u8 as i32).unwrap();
        let rec_sig =
            secp256k1::ecdsa::RecoverableSignature::from_compact(&sig_compact, sig_v).unwrap();
        let pub_key = s.recover_ecdsa(&msg, &rec_sig).unwrap();
        let public_key = Vec::from(&keccak256_hash(&pub_key.serialize_uncompressed()[1..])[12..]);

        if bridge_pool_instance
            .used_hashes_dict
            .get::<bool>(message_hash.as_str())
            .is_some()
        {
            return Err(Error::MessageAlreadyUsed);
        } else {
            bridge_pool_instance
                .used_hashes_dict
                .set(message_hash.as_str(), true);
        }

        let signer = hex::encode(public_key);

        if !bridge_pool_instance
            .signers_dict
            .get::<bool>(&signer)
            .ok_or(Error::NoValueInSignersDict)?
        {
            return Err(Error::InvalidSigner);
        }

        let client_addr = actor;
        runtime::call_versioned_contract::<()>(
            token,
            None,
            "transfer",
            runtime_args! {
                "recipient" => client_addr,
                AMOUNT => amount
            },
        );
        bridge_pool_instance.del_liquidity_generic_from_dict(
            token.to_formatted_string(),
            actor.as_account_hash().unwrap().to_string(),
            amount,
            bridge_pool_instance.get_dict(actor)?,
        )?;
        self.emit(BridgePoolEvent::TransferBySignature {
            signer,
            receiver,
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
        let res = bridge_pool_instance.check_signer(signer)?;
        Ok(res)
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
