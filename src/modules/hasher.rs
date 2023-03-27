// src/modules/hasher.rs
use sha3::{Digest, Keccak256};

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

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

pub fn encode_to_58base(input: &[u8]) -> String {

    let mut bytes = input.to_vec();
    let mut result = String::new();

    while !bytes.is_empty() {
        let mut carry = 0u32;

        for byte in bytes.iter_mut() {
            let value = (carry << 8) | (*byte as u32);
            *byte = (value / 58) as u8;
            carry = value % 58;
        }

        while carry > 0 {
            result.push(BASE58_ALPHABET.chars().nth((carry % 58) as usize).unwrap());
            carry /= 58;
        }

        while let Some(0) = bytes.first() {
            bytes.remove(0);
        }
    }

    for _ in 0..bytes.len() {
        result.push(BASE58_ALPHABET.chars().nth(0).unwrap());
    }

    result.chars().collect()
}

