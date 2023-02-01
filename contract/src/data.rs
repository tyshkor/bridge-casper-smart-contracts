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
    fees_dict: Dict,
    allowed_targets_dict: Dict,
}

impl BrigdePool {
    pub fn instance() -> BrigdePool {
        BrigdePool {
            liquidities_dict: Dict::instance(LIQUIDITIES_DICT),
            fees_dict: Dict::instance(FEES_DICT),
            allowed_targets_dict: Dict::instance(ALLOWED_TARGETS_DICT),
        }
    }

    pub fn init() {
        Dict::init(LIQUIDITIES_DICT);
        Dict::init(FEES_DICT);
        Dict::init(ALLOWED_TARGETS_DICT);
    }

    pub fn get_amount_staked_by_address(&self, address: &Key) -> Option<U256> {
        self.liquidities_dict.get(&key_to_str(address))
    }

    pub fn add_stake(&self, owner: &Key, amount: &U256) {
        let new_amount = if let Some(staked_amount) = self.get_amount_staked_by_address(owner) {
            staked_amount + amount
        } else {
            *amount
        };
        self.liquidities_dict.set(&key_to_str(owner), new_amount);
    }

    pub fn withdraw_stake(&self, owner: &Key, amount: &U256) {
        let staked_amount = self.get_amount_staked_by_address(owner).unwrap();
        let new_amount = staked_amount - amount;
        self.liquidities_dict.set(&key_to_str(owner), new_amount);
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
