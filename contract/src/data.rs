use crate::alloc::borrow::ToOwned;
use crate::error::Error;
use crate::event::BridgePoolEvent;
use crate::{address::Address, detail};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_call_stack},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::RuntimeArgs;
use casper_types::{runtime_args, system::CallStackElement, ContractPackageHash, URef, U256};
use contract_utils::{keccak::keccak256, Dict};

use k256::ecdsa::{
    recoverable::Signature as RecoverableSignature, signature::Signature as NonRecoverableSignature,
};

const ACCOUNT_HASH_LIQUIDITIES_DICT: &str = "account_hash_liquidities_dict";
const HASH_ADDR_LIQUIDITIES_DICT: &str = "hash_addr_liquidities_dict";
const ALLOWED_TARGETS_DICT: &str = "allowed_targets_dict";
const USED_HASHES_DICT: &str = "used_hashes_dict";
const SIGNERS_DICT: &str = "signers_dict";
const TOKEN_CONTRACT_PACKAGE_HASH_DICT_NAME: &str = "token_contract_package_hash_dict_name";
const BRIDGE_POOL_CONTRACT_PACKAGE_HASH: &str = "bridge_pool_contract_package_hash";

const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

const AMOUNT: &str = "amount";
const SIGNER: &str = "signer";
const TARGET_NETWORK: &str = "target_network";
const TARGET_ADDRESS: &str = "target_address";
const TOKEN: &str = "token";
const RECEIVER: &str = "receiver";
const ACTOR: &str = "actor";
const EVENT_TYPE: &str = "event_type";

const BRIDGE_LIQUIDITY_ADDED: &str = "bridge_liquidity_added";
const BRIDGE_LIQUIDITY_REMOVED: &str = "bridge_liquidity_removed";
const BRIDGE_SWAP: &str = "bridge_swap";
const BRIDGE_TRANSFER_BY_SIGNATURE: &str = "bridge_transfer_by_signature";

pub struct BridgePool {
    // dictionary to track client conected dictionaries
    account_hash_liquidities_dict: Dict,
    // dictionary to track external contracts' conected dictionaries
    hash_addr_liquidities_dict: Dict,
    // dictionary to track allowed targets
    allowed_targets_dict: Dict,
    // dictionary to track used hashes
    #[allow(unused)]
    used_hashes_dict: Dict,
    // dictionary to track signers
    signers_dict: Dict,
    token_contract_package_hash_dict_name: Dict,
}

impl BridgePool {
    pub fn instance() -> BridgePool {
        BridgePool {
            account_hash_liquidities_dict: Dict::instance(ACCOUNT_HASH_LIQUIDITIES_DICT),
            hash_addr_liquidities_dict: Dict::instance(HASH_ADDR_LIQUIDITIES_DICT),
            allowed_targets_dict: Dict::instance(ALLOWED_TARGETS_DICT),
            used_hashes_dict: Dict::instance(USED_HASHES_DICT),
            signers_dict: Dict::instance(SIGNERS_DICT),
            token_contract_package_hash_dict_name: Dict::instance(
                TOKEN_CONTRACT_PACKAGE_HASH_DICT_NAME,
            ),
        }
    }

    pub fn init() {
        Dict::init(ACCOUNT_HASH_LIQUIDITIES_DICT);
        Dict::init(HASH_ADDR_LIQUIDITIES_DICT);
        Dict::init(ALLOWED_TARGETS_DICT);
        Dict::init(USED_HASHES_DICT);
        Dict::init(SIGNERS_DICT);
        Dict::init(TOKEN_CONTRACT_PACKAGE_HASH_DICT_NAME);
    }

    // function to get liquidity already in pool by client address
    pub fn get_liquidity_added_by_client(
        &self,
        token_contract_hash: ContractPackageHash,
        client_address: Address,
    ) -> Result<U256, Error> {
        let client_string: String = TryInto::try_into(client_address)?;
        Ok(self.get_liquidity_added_by_client_generic(
            token_contract_hash.to_string(),
            client_string,
            self.get_dict(client_address)?,
        ))
    }

    pub fn get_liquidity_added_by_client_generic(
        &self,
        token_contract_hash: String,
        client: String,
        dict: &Dict,
    ) -> U256 {
        let mut res = U256::zero();
        if let Some(token_address) = dict.get::<String>(token_contract_hash.as_str()) {
            let clients_dict = Dict::instance(token_address.as_str());
            if let Some(amount) = clients_dict.get(client.as_str()) {
                res += amount;
            }
        }
        res
    }

    // add liquidity to the pool
    pub fn add_liquidity(
        &self,
        bridge_pool_contract_package_hash: ContractPackageHash,
        token_contract_package_hash: ContractPackageHash,
        client_address: Address,
        amount: U256,
    ) -> Result<(), Error> {
        self.pay_to(
            token_contract_package_hash,
            client_address,
            crate::address::Address::ContractPackage(bridge_pool_contract_package_hash),
            amount,
        );

        let client_string: String = TryInto::try_into(client_address)?;
        self.add_liquidity_generic(
            token_contract_package_hash.to_formatted_string(),
            client_string,
            amount,
            self.get_dict(client_address)?,
        );

        Ok(())
    }

    // generic function to handle the case of a client and a contract when adding liquidity
    pub fn add_liquidity_generic(
        &self,
        token_contract_hash: String,
        client: String,
        amount: U256,
        dict: &Dict,
    ) {
        if let Some(clients_dict_address) = dict.get::<String>(token_contract_hash.as_str()) {
            let clients_dict = Dict::instance(clients_dict_address.as_str());
            if let Some(client_amount) = clients_dict.get::<U256>(client.as_str()) {
                clients_dict.set(client.as_str(), client_amount + amount)
            } else {
                clients_dict.set(client.as_str(), amount)
            }
        } else {
            let client_dict_name = token_contract_hash.as_str();
            Dict::init(client_dict_name);
            let clients_dict = Dict::instance(client_dict_name);
            clients_dict.set(client.as_str(), amount);
            dict.set(token_contract_hash.as_str(), client_dict_name);
        }
    }

    // remove liquidity from the pool
    pub fn remove_liquidity(
        &self,
        token_contract_package_hash: ContractPackageHash,
        client_address: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let client_string: String = TryInto::try_into(client_address)?;
        self.pay_from_me(token_contract_package_hash, client_address, amount);
        self.del_liquidity_generic_from_dict(
            token_contract_package_hash.to_formatted_string(),
            client_string,
            amount,
            self.get_dict(client_address)?,
        )?;
        Ok(())
    }

    // generic function to handle the case of a client and a contract when removing liquidity
    fn del_liquidity_generic_from_dict(
        &self,
        token_contract_hash: String,
        client: String,
        amount: U256,
        dict: &Dict,
    ) -> Result<(), Error> {
        if let Some(clients_dict_address) = dict.get::<String>(token_contract_hash.as_str()) {
            let clients_dict = Dict::instance(clients_dict_address.as_str());
            if let Some(client_amount) = clients_dict.get::<U256>(client.as_str()) {
                if let Some(new_amount) = client_amount.checked_sub(amount) {
                    clients_dict.set(client.as_str(), new_amount);
                    Ok(())
                } else {
                    Err(Error::CheckedSubFail)
                }
            } else {
                Err(Error::ClientDoesNotHaveSpecificKindOfLiquidity)
            }
        } else {
            Err(Error::ClientDoesNotHaveAnyKindOfLiquidity)
        }
    }

    // function to add signer
    pub fn add_signer(&self, signer: String) {
        self.signers_dict.set(&signer, true)
    }

    // function to remvoe signer
    pub fn remove_signer(&self, signer: String) {
        self.signers_dict.remove::<bool>(&signer)
    }

    pub fn check_signer(&self, signer: String) -> Result<bool, Error> {
        let res = self
            .signers_dict
            .get::<bool>(&signer)
            .ok_or(Error::NoValueInSignersDict)?;
        Ok(res)
    }

    // withdraw liquidity from pool securely
    #[allow(clippy::too_many_arguments)]
    pub fn withdraw_signed(
        &self,
        token_contract_package_hash: ContractPackageHash,
        payee: String,
        amount: U256,
        chain_id: u64,
        salt_string: String,
        token_recipient: String,
        signature: alloc::vec::Vec<u8>,
        actor: Address,
    ) -> Result<String, Error> {
        let salt: [u8; 32] = hex::decode(salt_string)
            .map_err(|_| Error::SaltHexFail)?
            .try_into()
            .map_err(|_| Error::SaltWrongSize)?;

        let message_hash = hex::encode(keccak256(
            &hex::encode(keccak256(
                &[
                    token_contract_package_hash.to_formatted_string().as_bytes(),
                    payee.as_bytes(),
                    amount.to_string().as_bytes(),
                    token_recipient.as_bytes(),
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

        let public_key = contract_utils::keccak::ecdsa_recover(
            &hex::decode(message_hash.clone()).map_err(|_| Error::MessageHashHexDecodingFail)?[..],
            &signature_rec,
        )
        .map_err(|_| Error::EcdsaPublicKeyRecoveryFail)?;

        if self
            .used_hashes_dict
            .get::<bool>(message_hash.as_str())
            .is_some()
        {
            return Err(Error::MessageAlreadyUsed);
        } else {
            self.used_hashes_dict.set(message_hash.as_str(), true);
        }

        let signer_string = hex::encode(public_key);

        if !self
            .signers_dict
            .get::<bool>(&signer_string)
            .ok_or(Error::NoValueInSignersDict)?
        {
            return Err(Error::InvalidSigner);
        }

        let client_addr = actor;
        runtime::call_versioned_contract::<()>(
            token_contract_package_hash,
            None,
            "transfer",
            runtime_args! {
                "recipient" => client_addr,
                AMOUNT => amount
            },
        );
        self.del_liquidity_generic_from_dict(
            token_contract_package_hash.to_formatted_string(),
            actor.as_account_hash().unwrap().to_string(),
            amount,
            self.get_dict(actor)?,
        )?;
        Ok(signer_string)
    }

    // function to build signed message
    pub fn withdraw_signed_message(
        &self,
        token_contract_package_hash: ContractPackageHash,
        payee: String,
        amount: U256,
        salt: [u8; 32],
    ) -> [u8; 32] {
        let contract_package_hash_bytes = token_contract_package_hash.as_bytes();
        let payee_bytes = payee.as_bytes();
        let amount_string = amount.to_string();
        let amount_bytes = amount_string.as_bytes();

        let concatenated: Vec<u8> = [
            contract_package_hash_bytes,
            payee_bytes,
            amount_bytes,
            &salt[..],
        ]
        .concat();
        let data = &concatenated[..];
        contract_utils::keccak::keccak256(data)
    }

    pub fn signer_unique(
        &self,
        message_hash: String,
        signature: alloc::vec::Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        let signature_rec = if signature.len() == 65 {
            RecoverableSignature::from_bytes(&signature[..])
                .map_err(|_| Error::RecoverableSignatureTryFromFail)?
        } else {
            NonRecoverableSignature::from_bytes(&signature[..])
                .map_err(|_| Error::NonRecoverableSignatureTryFromFail)?
        };

        let public_key = contract_utils::keccak::ecdsa_recover(
            &hex::decode(message_hash.clone()).map_err(|_| Error::MessageHashHexDecodingFail)?[..],
            &signature_rec,
        )
        .map_err(|_| Error::EcdsaPublicKeyRecoveryFail)?;

        if self
            .used_hashes_dict
            .get::<bool>(message_hash.as_str())
            .is_some()
        {
            return Err(Error::MessageAlreadyUsed);
        } else {
            self.used_hashes_dict.set(message_hash.as_str(), true);
        }
        Ok(public_key)
    }

    // function to swap tokens from different pools
    pub fn swap(
        &self,
        from_address: Address,
        token_contract_package_hash: ContractPackageHash,
        target_token: String,
        amount: U256,
        target_network: U256,
    ) -> Result<(), Error> {
        let token_contract_package_hash_string = token_contract_package_hash.to_string();
        if let Some(token_name_from_dict) = self
            .token_contract_package_hash_dict_name
            .get::<String>(token_contract_package_hash_string.as_str())
        {
            if let Some(target_token_dict_address) = self
                .allowed_targets_dict
                .get::<String>(&(ALLOWED_TARGETS_DICT.to_owned() + token_name_from_dict.as_str()))
            {
                let target_token_dict = Dict::instance(target_token_dict_address.as_str());
                if let Some(dict_target_token) =
                    target_token_dict.get::<String>(&target_network.to_string())
                {
                    if dict_target_token != target_token {
                        return Err(Error::DictTargetTokenNotEqualTargetToken);
                    }
                } else {
                    return Err(Error::NoTargetNetworkDictForThisToken);
                }
            } else {
                return Err(Error::NoTargetTokenInAllowedTargetsDict);
            }
            self.pay_me(token_contract_package_hash, from_address, amount);
            Ok(())
        } else {
            Err(Error::NoTokenInTokenContractPackageHashDict)
        }
    }

    // function to allow target for swap
    pub fn allow_target(
        &self,
        token_contract_package_hash: ContractPackageHash,
        token_name: String,
        target_token: String,
        target_network: U256,
    ) -> Result<(), Error> {
        let token_contract_package_hash_string = token_contract_package_hash.to_string();
        if let Some(token_name_from_dict) = self
            .token_contract_package_hash_dict_name
            .get::<String>(token_contract_package_hash_string.as_str())
        {
            if token_name != token_name_from_dict {
                return Err(Error::WrongTokenName);
            }
            if let Some(target_token_dict_address) = self
                .allowed_targets_dict
                .get::<String>(&(ALLOWED_TARGETS_DICT.to_owned() + token_name.as_str()))
            {
                let target_token_dict = Dict::instance(target_token_dict_address.as_str());
                if target_token_dict
                    .get::<String>(&target_network.to_string())
                    .is_some()
                {
                    return Err(Error::AlreadyInThisTargetTokenDict);
                } else {
                    target_token_dict.set(&target_network.to_string(), target_token)
                }
            } else {
                let target_token_dict_name: &str =
                    &(ALLOWED_TARGETS_DICT.to_owned() + token_name.as_str());
                Dict::init(target_token_dict_name);

                let target_token_dict = Dict::instance(target_token_dict_name);
                target_token_dict.set(target_network.to_string().as_str(), target_token.as_str());

                self.allowed_targets_dict
                    .set(target_token_dict_name, target_token_dict_name.to_string());
            }
        } else {
            self.token_contract_package_hash_dict_name.set(
                token_contract_package_hash_string.as_str(),
                token_name.clone(),
            );
            let target_token_dict_name: &str =
                &(ALLOWED_TARGETS_DICT.to_owned() + token_name.as_str());
            Dict::init(target_token_dict_name);

            let target_token_dict = Dict::instance(target_token_dict_name);
            target_token_dict.set(target_network.to_string().as_str(), target_token.as_str());

            self.allowed_targets_dict
                .set(target_token_dict_name, target_token_dict_name.to_string());
        }

        Ok(())
    }

    // pay from any address to any address. Remember to approve the tokens beforehand
    fn pay_to(&self, token: ContractPackageHash, owner: Address, recipient: Address, amount: U256) {
        let args = runtime_args! {
            "owner" => owner,
            "recipient" => recipient,
            AMOUNT => amount
        };
        runtime::call_versioned_contract::<()>(token, None, "transfer_from", args);
    }

    // pay from any address to this contract. Remember to approve the tokens beforehand
    fn pay_me(&self, token: ContractPackageHash, spender: Address, amount: U256) {
        let bridge_pool_contract_package_hash = runtime::get_key(BRIDGE_POOL_CONTRACT_PACKAGE_HASH)
            .unwrap_or_revert_with(Error::MissingContractPackageHash)
            .into_hash()
            .map(ContractPackageHash::new)
            .unwrap_or_revert_with(Error::InvalidContractPackageHash);
        self.pay_to(
            token,
            spender,
            crate::address::Address::ContractPackage(bridge_pool_contract_package_hash),
            amount,
        )
    }

    fn pay_from_me(&self, token: ContractPackageHash, recipient: Address, amount: U256) {
        let args = runtime_args! {
            "recipient" => recipient,
            AMOUNT => amount
        };
        runtime::call_versioned_contract::<()>(token, None, "transfer", args);
    }

    fn get_dict(&self, client_address: Address) -> Result<&Dict, Error> {
        match client_address {
            Address::Account(_) => Ok(&self.account_hash_liquidities_dict),
            Address::ContractPackage(_) => Ok(&self.hash_addr_liquidities_dict),
            Address::ContractHash(_) => Err(Error::UnexpectedContractHash),
        }
    }
}

// function to return contract package hash in case it's possible
pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}

// function to emit an event
pub fn emit(event: &BridgePoolEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        BridgePoolEvent::BridgeLiquidityAdded {
            actor,
            token,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, BRIDGE_LIQUIDITY_ADDED.to_string());
            param.insert(ACTOR, (*actor).try_into().unwrap());
            param.insert(TOKEN, token.to_string());
            param.insert(AMOUNT, amount.to_string());
            events.push(param);
        }
        BridgePoolEvent::BridgeLiquidityRemoved {
            actor,
            token,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, BRIDGE_LIQUIDITY_REMOVED.to_string());
            param.insert(ACTOR, (*actor).try_into().unwrap());
            param.insert(TOKEN, token.to_string());
            param.insert(AMOUNT, amount.to_string());
            events.push(param);
        }
        BridgePoolEvent::BridgeSwap {
            actor,
            token,
            target_network,
            target_address,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, BRIDGE_SWAP.to_string());
            param.insert(ACTOR, (*actor).try_into().unwrap());
            param.insert(TOKEN, token.to_string());
            param.insert(TARGET_NETWORK, target_network.to_string());
            param.insert(TARGET_ADDRESS, target_address.clone());
            param.insert(AMOUNT, amount.to_string());
            events.push(param);
        }
        BridgePoolEvent::TransferBySignature {
            signer,
            receiver,
            token,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, BRIDGE_TRANSFER_BY_SIGNATURE.to_string());
            param.insert(SIGNER, signer.clone());
            param.insert(TOKEN, token.to_string());
            param.insert(RECEIVER, receiver.to_string());
            param.insert(AMOUNT, amount.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
