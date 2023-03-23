// src/modules/ecdsa.rs

use k256::Secp256k1;
use k256::elliptic_curve::{SecretKey, generic_array::GenericArray};

#[path = "hasher.rs"] mod hasher;
#[path = "verifier.rs"] mod verifier;

// Generate a new public key from given private key
pub fn gen_new_keys() -> (SecretKey<Secp256k1>, k256::PublicKey) {
    let seed = verifier::new_u256();
    let private_key_bytes = verifier::generate_new_seed(seed);
    // Create a `SecretKey` instance from the byte array
    let private_key : SecretKey<Secp256k1> = SecretKey::from_slice(&private_key_bytes.0).unwrap();
    let public_key = private_key.public_key();

    (private_key, public_key)
}

pub fn gen_from_words(words: [String; 12]) -> (SecretKey<Secp256k1>, k256::PublicKey) {
    
}