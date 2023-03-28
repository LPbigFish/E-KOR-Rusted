// src/main.rs

use structures::Wallet;


#[path = "modules/hasher.rs"] mod hasher;
#[path = "modules/verifier.rs"] mod verifier;
#[path = "modules/ecdsa.rs"] mod ecdsa;
#[path = "modules/structures.rs"] mod structures;


fn main() {
    
    let wallet = Wallet::new();

    println!("{}", wallet.to_string());
}
