use tiny_keccak::{Hasher, Keccak};
use k256::ecdsa::recoverable::{Signature as RecoverableSignature};
use alloc::vec::Vec;

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(data);
    hasher.finalize(&mut output);

    output
}

pub fn ecdsa_recover(hash: &[u8], sig: &RecoverableSignature) -> Result<Vec<u8>, secp256k1::Error> {
    let s = secp256k1::Secp256k1::new();
    let msg = secp256k1::Message::from_slice(hash).unwrap();
    let mut sig_compact: Vec<u8> = sig.r().to_bytes().to_vec().clone();
    sig_compact.extend(&sig.s().to_bytes().to_vec());
    let id_u8: u8 = From::from(sig.recovery_id().clone());
    let sig_v = secp256k1::recovery::RecoveryId::from_i32(id_u8 as i32).unwrap();
    let rec_sig = secp256k1::recovery::RecoverableSignature::from_compact(&sig_compact, sig_v);
    match rec_sig {
        Ok(r) => {
            match s.recover(&msg, &r) {
                Ok(pub_key) => {
                    let pk_bytes_raw: [u8; 65] = pub_key.serialize_uncompressed();
                    Ok(public_to_address(&pk_bytes_raw[1..]))
                }
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    }
}

pub fn public_to_address(public: &[u8]) -> Vec<u8> {
    let hash = keccak256_hash(public);
    Vec::from(&hash[12..])
}

pub fn keccak256_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    let mut resp: [u8; 32] = Default::default();
    hasher.finalize(&mut resp);
    resp.iter().cloned().collect()
}
