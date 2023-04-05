// src/main.rs

use crate::structures::wallet::Wallet;

#[path = "modules/hasher.rs"] mod hasher;
#[path = "modules/verifier.rs"] mod verifier;
#[path = "modules/ecdsa.rs"] mod ecdsa;
#[path = "structures.rs"] mod structures;


fn main() {
    
    let wallet = Wallet::new();

    println!("{}", wallet.to_string());
}
