// src/modules/hasher.rs
use sha3::{Digest, Keccak256};


pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();

    let mut output = [0u8; 32];

    hasher.update(input);

    output.copy_from_slice(hasher.finalize().as_slice());

    output
}

pub fn sha3_224(input: &[u8]) -> [u8; 28] {
    let mut hasher = sha3::Sha3_224::new();

    let mut output = [0u8; 28];

    hasher.update(input);

    output.copy_from_slice(hasher.finalize().as_slice());

    output
}

pub fn hash_to_string(hash: &[u8]) -> String {
    let string = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    string
}