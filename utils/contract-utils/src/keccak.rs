use alloc::string::String;
use tiny_keccak::{Hasher, Keccak};

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(data);
    hasher.finalize(&mut output);

    output
}

// pub fn signer_unique() -> String {

// }

// pub fn signer() -> ([u8; 32], String) {
    
// }