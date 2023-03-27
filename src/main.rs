// src/main.rs


#[path = "modules/hasher.rs"] mod hasher;
#[path = "modules/verifier.rs"] mod verifier;
#[path = "modules/ecdsa.rs"] mod ecdsa;
#[path = "modules/structures.rs"] mod structures;


fn main() {
    
    let private_key = ecdsa::generate_private_key();

    println!("{:?}", hasher::hash_to_string(private_key.as_slice()));

    let public_key = ecdsa::generate_public_key(private_key.as_slice());

    let hash = hasher::keccak256(b"Super secret message");

    let fake_hash = hasher::keccak256(b"Super secret message 2");

    let signature = ecdsa::sign_message(private_key.as_slice(), hash.as_slice());

    let public_key_compressed = ecdsa::compress_pub(public_key);

    println!("{:?}", hasher::hash_to_string(public_key_compressed.as_slice()));

    println!("{:?}", ecdsa::verify_signature(&public_key_compressed, &hash, &signature));
}
