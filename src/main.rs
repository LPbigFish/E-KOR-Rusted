// src/main.rs

#[path = "modules/hasher.rs"] mod hasher;
#[path = "modules/verifier.rs"] mod verifier;


fn main() {
    //foreach u8 in [u8] convert to char
    let hash = hasher::keccak256(b"Hello, world!");

    let array = verifier::new_u256();

    let seed = verifier::generate_new_seed(array);

    println!("{:?}", hasher::hash_to_string(&seed.0));
    println!("{:?}", hasher::hash_to_string(&seed.1));
}
