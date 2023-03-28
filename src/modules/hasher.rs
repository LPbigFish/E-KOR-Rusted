use num::{BigUint, Zero, bigint::ToBigUint, ToPrimitive};
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

pub fn encode_base58(input: &[u8]) -> String {
    let mut num = BigUint::from_bytes_be(input);
    let mut encoded = String::new();

    while num > BigUint::zero() {
        let rem = &num % 58u32;
        num = num / 58u32;
        let ch = BASE58_ALPHABET.chars().nth(rem.to_usize().unwrap()).unwrap();
        encoded.push(ch);
    }

    for b in input.iter() {
        if *b == 0u8 {
            encoded.push(BASE58_ALPHABET.chars().nth(0).unwrap());
        } else {
            break;
        }
    }

    encoded.chars().rev().collect()
}

pub fn decode_base58(input: &str) -> Option<Vec<u8>> {
    let mut num = BigUint::zero();
    let base = BigUint::from(58u32);
    let zero = BigUint::zero();
    let mut leading_zeros = 0;

    for ch in input.chars() {
        let digit = BASE58_ALPHABET.find(ch)?;
        num = num * &base + digit.to_biguint().unwrap();
    }

    let mut result = num.to_bytes_be();

    for b in input.bytes() {
        if b == BASE58_ALPHABET.as_bytes()[0] {
            leading_zeros += 1;
        } else {
            break;
        }
    }

    result.reverse();
    for _ in 0..leading_zeros {
        result.insert(0, 0);
    }

    result.reverse();
    Some(result)
}

pub fn create_address(comp_public_key: &[u8]) -> String {
    let mut address = [0u8; 33];
    let hash = keccak256(comp_public_key);
    let hash = sha3_224(&hash);

    address[0] = 0x00;

    address[1..29].copy_from_slice(&hash);

    let checksum = &keccak256(&address[0..28])[0..5];

    address[28..].copy_from_slice(checksum);

    encode_base58(address.as_slice())
}