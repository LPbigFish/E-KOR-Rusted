// src/modules/ecdsa.rs

use num::bigint::BigUint;

pub fn get_g() -> ([u8;32], [u8;32]) {

    let mut g_x = [0u8; 32];
    let mut g_y = [0u8; 32];

    hex::decode_to_slice("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", &mut g_x).unwrap();
    hex::decode_to_slice("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", &mut g_y).unwrap();

    (g_x, g_y)
}

pub fn get_n() -> [u8; 32] {
    let mut n = [0u8; 32];
    hex::decode_to_slice("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", &mut n).unwrap();
    n
}

pub fn multiply(x : &[u8], y : &[u8]) -> Vec<u8> {
    let x = BigUint::from_bytes_be(x);
    let y = BigUint::from_bytes_be(y);

    let result = x * y;

    let result = result.to_bytes_be();

    //convert result to slice
    result
}

pub fn double(x : &[u8]) -> Vec<u8> {
    let x = BigUint::from_bytes_be(x);

    let result : BigUint = x * 2u8;

    let result = result.to_bytes_be();

    //convert result to slice
    result
}