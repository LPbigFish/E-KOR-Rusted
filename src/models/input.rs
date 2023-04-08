// src/modules/models/input.rs

pub struct Input {
    txid: [u8; 32],
    vout: u32,
    script_length: u32,
    script: Vec<u8>
}