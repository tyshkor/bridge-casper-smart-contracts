use alloc::string::String;
use casper_types::{ContractPackageHash, U256};

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
        target_token: String,
        // client address
        target_address: Address,
        amount: U256,
    },
    TransferBySignature {
        signer: Address,
        reciever: String,
        token: ContractPackageHash,
        amount: U256,
    },
}
