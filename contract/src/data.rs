use crate::detail;
use crate::event::BridgePoolEvent;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, URef, U256};
use contract_utils::{get_key, key_to_str, set_key, Dict};

const LIQUIDITIES_DICT: &str = "liquidities_dict";
const FEES_DICT: &str = "fees_dict";
const ALLOWED_TARGETS_DICT: &str = "allowed_targets_dict";

const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

const NAME: &str = "name";
const ADDRESS: &str = "address";

pub struct BrigdePool {
    liquidities_dict: Dict,
    // fees_dict: Dict,
    // allowed_targets_dict: Dict,
}

impl BrigdePool {
    pub fn instance() -> BrigdePool {
        BrigdePool {
            liquidities_dict: Dict::instance(LIQUIDITIES_DICT),
            // fees_dict: Dict::instance(FEES_DICT),
            // allowed_targets_dict: Dict::instance(ALLOWED_TARGETS_DICT),
        }
    }

    pub fn init() {
        Dict::init(LIQUIDITIES_DICT);
        // Dict::init(FEES_DICT);
        // Dict::init(ALLOWED_TARGETS_DICT);
    }

    pub fn get_liquidity_added_by_client(&self, token_contract_hash: String, client: String) -> Option<U256> {
        let token_address: String  = self.liquidities_dict.get(token_contract_hash.as_str())?;
        let clients_dict = Dict::instance(token_address.as_str());
        clients_dict.get(client.as_str())
    }

    pub fn add_liquidity(&self, token_contract_hash: String, client: String, amount: U256) {
        if let Some(clients_dict_address) = self.liquidities_dict.get::<String>(token_contract_hash.as_str()) {
            let clients_dict = Dict::instance(clients_dict_address.as_str());
            if let Some(client_amount) = clients_dict.get::<U256>(client.as_str()) {
                clients_dict.set(client.as_str(), client_amount + amount)
            } else {
                clients_dict.set(client.as_str(), amount)
            }
        } else {
            let client_dict_name_string = alloc::format!("{token_contract_hash}_{client}");
            let client_dict_name = client_dict_name_string.as_str();
            Dict::init(client_dict_name);
            let clients_dict = Dict::instance(client_dict_name);
            clients_dict.set(client.as_str(), amount);
            self.liquidities_dict.set(token_contract_hash.as_str(), client_dict_name);
        }
    }

    pub fn remove_liquidity(&self, token_contract_hash: String, client: String, amount: U256) {
        if let Some(clients_dict_address) = self.liquidities_dict.get::<String>(token_contract_hash.as_str()) {
            let clients_dict = Dict::instance(clients_dict_address.as_str());
            if let Some(client_amount) = clients_dict.get::<U256>(client.as_str()) {
                let new_amount = client_amount.checked_sub(amount).unwrap(); //handle error
                clients_dict.set(client.as_str(), new_amount)
            } else {
                // error
            }
        } else {
            // error
        }
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn address() -> String {
    get_key(ADDRESS).unwrap_or_revert()
}

pub fn set_address(address: String) {
    set_key(ADDRESS, address);
}

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
            param.insert("event_type", "bridge_liquidity_added".to_string());
            if actor.as_account_hash().is_some() {
                param.insert("actor", actor.as_account_hash().unwrap().to_string());
            } else {
                param.insert(
                    "actor",
                    actor.as_contract_package_hash().unwrap().to_string(),
                );
            };
            param.insert("token", token.to_string());
            param.insert("amount", amount.to_string());
            events.push(param);
        }
        BridgePoolEvent::BridgeLiquidityRemoved {
            actor,
            token,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "bridge_liquidity_removed".to_string());
            if actor.as_account_hash().is_some() {
                param.insert("actor", actor.as_account_hash().unwrap().to_string());
            } else {
                param.insert(
                    "actor",
                    actor.as_contract_package_hash().unwrap().to_string(),
                );
            };
            param.insert("token", token.to_string());
            param.insert("amount", amount.to_string());
            events.push(param);
        }
        BridgePoolEvent::BridgeSwap {
            actor,
            token,
            target_network,
            target_token,
            target_address,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "bridge_swap".to_string());
            if actor.as_account_hash().is_some() {
                param.insert("actor", actor.as_account_hash().unwrap().to_string());
            } else {
                param.insert(
                    "actor",
                    actor.as_contract_package_hash().unwrap().to_string(),
                );
            };
            param.insert("token", token.to_string());
            param.insert("target_network", target_network.to_string());
            param.insert("target_token", target_token.to_string());
            if target_address.as_account_hash().is_some() {
                param.insert(
                    "target_address",
                    target_address.as_account_hash().unwrap().to_string(),
                );
            } else {
                param.insert(
                    "target_address",
                    target_address
                        .as_contract_package_hash()
                        .unwrap()
                        .to_string(),
                );
            };
            param.insert("amount", amount.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
