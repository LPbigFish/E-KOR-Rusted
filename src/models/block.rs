// src/modules/models/block.rs

use std::{time::{SystemTime, UNIX_EPOCH}, thread, sync::{Mutex, Arc}};

use num::{bigint::Sign, BigInt};

use crate::hasher;

use super::transaction::Transaction;

pub struct Block {
    version: u8,
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    proof: [u8; 28],
    difficulty: u32,

    root: [u8; 32],
    previous_hash: [u8; 32],
    block_hash: [u8; 32],
    nonce: u64,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, proof: [u8; 28], difficulty: u32, previous_hash: [u8; 32]) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let mut block = Block {
            version: 1,
            index,
            timestamp,
            transactions,
            proof,
            difficulty,
            root: [0u8; 32],
            previous_hash,
            block_hash: [0u8; 32],
            nonce: 0,
        };

        block.root = block.calculate_root();
        let temp = block.calculate_hash();
        block.block_hash = temp.0;
        block.nonce = temp.1;

        block
    }

    pub fn calculate_root(&self) -> [u8; 32] {
        let mut merkle_proofs: Vec<[u8;32]> = self.transactions.iter().map(|tx| tx.hash).collect();
        //hash 2 transactions at a time together
        if merkle_proofs.len() == 0 {
            return hasher::keccak256(b"empty");
        }
        while merkle_proofs.len() > 1 {
            let len = merkle_proofs.len();
            for i in (0..merkle_proofs.len()).step_by(2) {
                let mut hash = [0u8; 32];
                let mut buffer = [0u8; 64];
                if i + 1 < self.transactions.len() {
                    buffer[..32].copy_from_slice(&self.transactions[i].hash);
                    buffer[32..].copy_from_slice(&self.transactions[i + 1].hash);
                    hash = hasher::keccak256(&buffer);
                } else {
                    hash = hasher::keccak256(&self.transactions[i].hash);
                }
                merkle_proofs.push(hash);
            }

            merkle_proofs.drain(..len);
        }
        

        merkle_proofs[0]
    }

    pub fn calculate_hash(&self) -> ([u8; 32], u64) {
        let mut buffer = [0u8; 160];
        let target = [255u8;32];
        let mut handles = vec![];
        let num_threads = 4;

        let proof = Arc::new(Mutex::new(Vec::<u8>::new()));
        
        let target = (BigInt::from_bytes_be(Sign::Plus, &target) / BigInt::from(self.difficulty)).to_bytes_be().1;

        buffer[..32].copy_from_slice(&self.previous_hash);
        buffer[32..64].copy_from_slice(&self.root);
        buffer[64..96].copy_from_slice(&self.index.to_be_bytes());
        
        for i in 0..num_threads {
            let mut nonce = i as u64;
            let mut candidate = [0u8; 32];
            let mut buffer = buffer.clone();
            let target = target.clone();
            let proof = proof.clone();

        let thread_hendle = thread::spawn(move || {          
            let mut result = [0u8; 32];  
            loop {
                buffer[96..].copy_from_slice(&nonce.to_be_bytes());
                candidate = hasher::keccak256(&buffer);
                if candidate.as_slice() < target.as_slice() {
                    result = candidate;
                    break;
                }
                proof.lock().unwrap()[nonce as usize] = candidate[0];
                nonce += num_threads as u64;
            }
            (result, nonce)
            });

            handles.push(thread_hendle);
        }

        for handle in handles {
            match handle.join() {
                Ok(output) => return output,
                Err(_) => (),
            }
        }

        ([0u8; 32], 0)
    }

    pub fn new_existing(index: u32, timestamp: u64, transactions: Vec<Transaction>, proof: [u8; 28], difficulty: u32, previous_hash: [u8; 32], root: [u8; 32], block_hash: [u8; 32], nonce: u64) -> Self {
        Block {
            version: 1,
            index,
            timestamp,
            transactions,
            proof,
            difficulty,
            root,
            previous_hash,
            block_hash,
            nonce
        }
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        //convert everything in the block to hex
        let mut block_string = String::new();
        block_string.push_str(&hex::encode(&self.version.to_be_bytes()));
        block_string.push_str(&hex::encode(&self.index.to_be_bytes())); 
        block_string.push_str(&hex::encode(&self.timestamp.to_be_bytes()));
        block_string.push_str(&hex::encode(&self.proof));
        block_string.push_str(&hex::encode(&self.difficulty.to_be_bytes()));
        block_string.push_str(&hex::encode(&self.previous_hash));
        block_string.push_str(&hex::encode(&self.root));
        block_string.push_str(&hex::encode(&self.block_hash));
        block_string.push_str(&hex::encode(&self.nonce.to_be_bytes()));

        block_string
    }
}