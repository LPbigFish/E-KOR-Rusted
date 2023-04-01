use crate::{ecdsa, hasher};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: [u8; 32],
    pub y: [u8; 32],
}



#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: [u8; 32],
    pub public_key: Point,
    pub public_key_compressed: [u8; 33],
    pub address: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let private_key = ecdsa::generate_private_key();
        let public_key = ecdsa::generate_public_key(private_key.as_slice());
        let public_key_compressed = ecdsa::compress_pub(public_key);
        let address = hasher::create_address(&public_key_compressed);

        Wallet {
            private_key,
            public_key,
            public_key_compressed,
            address,
        }
    }
}

impl ToString for Wallet {
    fn to_string(&self) -> String {
        let mut string = String::new();

        string.push_str("Private Key: ");
        string.push_str(&hasher::hash_to_string(&self.private_key));
        string.push_str("\n");
        string.push_str("Public Key: \n");
        string.push_str("X: ");
        string.push_str(&hasher::hash_to_string(&self.public_key.x));
        string.push_str("\nY: ");
        string.push_str(&hasher::hash_to_string(&self.public_key.y));
        string.push_str("\n");
        string.push_str("Public Key Compressed: ");
        string.push_str(&hasher::hash_to_string(&self.public_key_compressed));
        string.push_str("\n");
        string.push_str("Address: ");
        string.push_str(&self.address);

        string
    }
}