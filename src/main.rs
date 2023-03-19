// src/main.rs

#[path = "modules/hasher.rs"] mod hasher;

fn main() {
    //foreach u8 in [u8] convert to char
    let hash = hasher::keccak256("Hello, world!");

    println!("{:?}", hash);
}
