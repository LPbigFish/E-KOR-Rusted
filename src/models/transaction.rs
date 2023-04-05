// src/modules/transaction.rs

pub struct Transaction {
    version: u8,
    input_count: u8,
    inputs: Vec<[u8; 32]>,
    output_count: u8,
    outputs: Vec<[u8; 32]>,
    lock_time: u32,
    pub(crate) hash: [u8; 32],
}