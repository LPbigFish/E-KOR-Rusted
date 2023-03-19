// src/modules/hasher.rs
use tiny_keccak::{Keccak, Hasher};


pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

pub fn hash_to_string(hash: &[u8]) -> String {
    let string = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    string
}