// src/main.rs

#[path = "modules/hasher.rs"] mod hasher;
#[path = "modules/verifier.rs"] mod verifier;
#[path = "modules/ecdsa.rs"] mod ecdsa;


fn main() {
    //foreach u8 in [u8] convert to char
    let hash = hasher::keccak256(b"Hello, world!");

    let array = verifier::new_u256();

    let seed = verifier::generate_new_seed(array);

    println!("{:?}", hasher::hash_to_string(&seed.0));
    println!("{:?}", hasher::hash_to_string(&seed.1));
    println!("{:?}", hasher::hash_to_string(&hash));
    println!("{:?}", hasher::hash_to_string(&ecdsa::get_n()));
    println!("{:?}\n{:?}", hasher::hash_to_string(&ecdsa::get_g().0), hasher::hash_to_string(&ecdsa::get_g().1));

    let result = ecdsa::multiply(&ecdsa::get_g().0, &array);

    println!("{:?}", hasher::hash_to_string(&result));
}
