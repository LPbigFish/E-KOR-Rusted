// src/modules/hasher.rs
use tiny_keccak::{Keccak, Hasher};

pub fn keccak256(input: &str) -> String {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input.as_bytes());
    hasher.finalize(&mut output);
    output.iter().map(|b| format!("{:02x}", b)).collect()
}