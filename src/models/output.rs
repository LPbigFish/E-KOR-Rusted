// src/modules/models/output.rs

pub struct Output {
    value: u32,
    script_length: u8,
    script: Vec<u8>
}

impl Output {
    pub fn new(value: u32, script: Vec<u8>) -> Self {
        Output {
            value,
            script_length: script.len() as u8,
            script
        }
    }
}