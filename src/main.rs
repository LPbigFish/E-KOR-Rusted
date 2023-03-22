// src/main.rs

use num::BigInt;

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
    println!("{:?}", ecdsa::get_n());
    println!("{:?}\n{:?}", ecdsa::get_g().x, ecdsa::get_g().y);

    let result = ecdsa::inverse(BigInt::from(2));

    println!("{:?}", result);

    println!("{:?}", ecdsa::multiply(bigint!("2")));

    print!("{:?}", ecdsa::get_p());
}
