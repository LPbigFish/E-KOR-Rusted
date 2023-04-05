// src/modules/transaction.rs

use crate::hasher;

#[derive(Debug, Clone)]
pub struct Transaction {
    version: u8,
    input_count: u8,
    inputs: Vec<Vec<u8>>,
    output_count: u8,
    outputs: Vec<Vec<u8>>,
    lock_time: u64,
    pub(crate) hash: [u8; 32]
}

impl Transaction {
    pub fn new_empty() -> Self {
        let mut tx = Transaction {
            version: 1,
            input_count: 0,
            inputs: Vec::new(),
            output_count: 0,
            outputs: Vec::new(),
            lock_time: 0,
            hash: [0u8; 32]
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    pub fn new(version: u8, inputs: Vec<Vec<u8>>, outputs: Vec<Vec<u8>>, lock_time: u64) -> Self {
        let mut tx = Transaction {
            version,
            input_count: inputs.len() as u8,
            inputs,
            output_count: outputs.len() as u8,
            outputs,
            lock_time,
            hash: [0u8; 32]
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    pub fn new_coinbase(version: u8, output: Vec<u8>, lock_time: u64) -> Self {
        let mut tx = Transaction {
            version,
            input_count: 0,
            inputs: Vec::new(),
            output_count: 1,
            outputs: vec![output],
            lock_time,
            hash: [0u8; 32]
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut buffer = Vec::<&[u8]>::new();

        let binding = self.version.to_be_bytes();
        buffer.push(binding.as_slice());
        let binding = self.input_count.to_be_bytes();
        buffer.push(&binding);
        for input in self.inputs.iter() {
            buffer.push(input);
        }
        let binding = self.output_count.to_be_bytes();
        buffer.push(&binding);
        for output in self.outputs.iter() {
            buffer.push(output);
        }
        let binding = self.lock_time.to_be_bytes();
        buffer.push(&binding);

        let mut output = [0u8; 32];
        output.copy_from_slice(hasher::keccak256(&buffer.concat()).as_slice());

        output
    }

    pub fn to_hex(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&self.version.to_be_bytes());
        result.extend_from_slice(&self.input_count.to_be_bytes());
        for input in self.inputs.iter() {
            result.extend_from_slice(input);
        }
        result.extend_from_slice(&self.output_count.to_be_bytes());
        for output in self.outputs.iter() {
            result.extend_from_slice(output);
        }
        result.extend_from_slice(&self.lock_time.to_be_bytes());

        result
    } 
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&hasher::hash_to_string(&self.to_hex()));

        output
    }
}