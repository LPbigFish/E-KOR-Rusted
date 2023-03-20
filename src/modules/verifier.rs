// src/modules/verifier.rs

#[path = "hasher.rs"] mod hasher;

use hmac::{Hmac, Mac};
use rand_core::RngCore;
use sha2::Sha512;

pub fn new_u256() -> [u8; 32] {
    let mut u256 = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut u256);
    u256
}

pub fn generate_new_seed(seed: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    type Hmac256 = Hmac::<Sha512>;

    let mut hmac = Hmac256::new_from_slice(&seed).unwrap();
    hmac.update(b"e-kor");

    let result = hmac.finalize().into_bytes().to_vec();

    let (mut array1, mut array2) = ([0u8; 32], [0u8; 32]);
    array1.clone_from_slice(&result[..32]);
    array2.clone_from_slice(&result[32..]);
    (array1, array2)
}