// src/main.rs

use crate::structures::{wallet::Wallet, transaction::Transaction};

#[path = "modules/hasher.rs"] mod hasher;
#[path = "modules/verifier.rs"] mod verifier;
#[path = "modules/ecdsa.rs"] mod ecdsa;
#[path = "modules/structures.rs"] mod structures;


fn main() {
    
    let wallet = Wallet::new();

    println!("{}", wallet.to_string());

    let tx = Transaction::new_empty();

    println!("{}", tx.to_string());
}
