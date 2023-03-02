use casper_types::{account::AccountHash, ContractPackageHash, U256};

use crate::address::Address;

pub enum BridgePoolEvent {
    BridgeLiquidityAdded {
        actor: Address,
        token: ContractPackageHash,
        amount: U256,
    },
    BridgeLiquidityRemoved {
        actor: Address,
        token: ContractPackageHash,
        amount: U256,
    },
    BridgeSwap {
        actor: Address,
        token: ContractPackageHash,
        target_network: U256,
        target_token: ContractPackageHash,
        target_address: Address,
        amount: U256,
    },
}
