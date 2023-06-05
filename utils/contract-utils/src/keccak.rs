use core::convert::TryInto;

use alloc::{string::String, vec::Vec};
use k256::ecdsa::recoverable::Signature as RecoverableSignature;
use secp256k1::{Message, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(data);
    hasher.finalize(&mut output);

    output
}

pub fn ecdsa_recover(hash: &[u8], sig: &RecoverableSignature) -> Result<Vec<u8>, secp256k1::Error> {
    let s = Secp256k1::new();
    let msg = Message::from_slice(hash).unwrap();
    let mut sig_compact: Vec<u8> = sig.r().to_bytes().to_vec().clone();
    sig_compact.extend(&sig.s().to_bytes().to_vec());
    let id_u8: u8 = From::from(sig.recovery_id().clone());
    let sig_v = secp256k1::ecdsa::RecoveryId::from_i32(id_u8 as i32).unwrap();
    let rec_sig = secp256k1::ecdsa::RecoverableSignature::from_compact(&sig_compact, sig_v)?;
    let pub_key =  s.recover_ecdsa(&msg, &rec_sig)?;
    Ok(Vec::from(&keccak256_hash(&pub_key.serialize_uncompressed()[1..])[12..]))
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

pub fn message_hash(
    token_contract_package_hash: String,
    payee: String,
    amount: String,
    chain_id: u64,
    salt: [u8; 32],
    token_recipient: String,
) -> String {

    hex::encode(keccak256(&[
        token_contract_package_hash.as_bytes(),
        payee.as_bytes(),
        amount.as_bytes(),
        token_recipient.as_bytes(),
        &chain_id.to_be_bytes(),
        &salt,
    ]
    .concat()[..]))
}

pub fn ecdsa_sign(hash: &[u8], private_key: &[u8]) -> [u8; 65] {
    let s = Secp256k1::signing_only();
    let msg = Message::from_slice(hash).unwrap();
    let key = SecretKey::from_slice(private_key).unwrap();
    let res = s.sign_ecdsa_recoverable(&msg, &key);
    let (v, sig_bytes) = s.sign_ecdsa_recoverable(&msg, &key).serialize_compact();
    let r = hex::encode(&sig_bytes[..32]);
    let s = hex::encode(&sig_bytes[32..64]);

    let mut vec = sig_bytes.to_vec();
    vec.push(v.to_i32() as u8);

    let slice = vec.as_slice();

    let mut vec1 = sig_bytes[..32].to_vec();
    let mut vec2 = sig_bytes[32..].to_vec();

    vec1.append(&mut vec2);
    vec1.push(v.to_i32() as u8);

    vec1.try_into().unwrap()
}
