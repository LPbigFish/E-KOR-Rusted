// src/modules/ecdsa.rs

use k256::{SecretKey, FieldBytes, elliptic_curve::sec1::{ToEncodedPoint, EncodedPoint}, Secp256k1, ecdsa::{signature::Signer, SigningKey, Signature, self, VerifyingKey}, schnorr::signature::{self, Verifier}, PublicKey};
use crate::structures::Point;
use crate::verifier;

pub fn generate_private_key() -> [u8; 32] {
    let seed = verifier::generate_new_seed(verifier::new_u256()).0;
    let secp = SecretKey::from_bytes(&FieldBytes::from(seed)).unwrap();
    
    let mut a1 = [0u8;32];

    a1.copy_from_slice(&secp.to_bytes().to_vec()[..32]);

    a1
}

pub fn generate_public_key(private_key: &[u8]) -> Point {
    let private_key : [u8;32] = private_key.try_into().unwrap();
    let secp = SecretKey::from_bytes(&FieldBytes::from(private_key)).unwrap();
    let public_key = secp.public_key();
    
    let uncompressed : EncodedPoint<Secp256k1> = public_key.to_encoded_point(false);

    let mut a1 = [0u8;32];
    let mut a2 = [0u8;32];
    a1.copy_from_slice(&uncompressed.x().unwrap().to_vec()[..32]);
    a2.copy_from_slice(&uncompressed.y().unwrap().to_vec()[..32]);
    Point {
            x: a1,
            y: a2,
        }
}

pub fn compress_pub(pub_key: Point) -> [u8; 33] {
    let uncomp = EncodedPoint::<Secp256k1>::from_affine_coordinates(&FieldBytes::from(pub_key.x), &FieldBytes::from(pub_key.y), true);

    let comp : EncodedPoint<Secp256k1> = uncomp.compress();

    let mut a1 = [0u8;33];

    a1.copy_from_slice(&comp.as_bytes().to_vec()[..33]);

    a1
}

pub fn sign_message(private_key: &[u8], message: &[u8]) -> [u8; 64] {
    let private_key : [u8;32] = private_key.try_into().unwrap();
    let secp = SecretKey::from_bytes(&FieldBytes::from(private_key)).unwrap();
    let secp = SigningKey::from(secp);
    let sign: Signature = secp.sign(message);
    let mut a1 = [0u8;64];
    a1.copy_from_slice(&sign.to_bytes().to_vec()[..64]);
    a1
}

pub fn verify_signature(compressed_pub: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let verifing_key = VerifyingKey::from_encoded_point(&EncodedPoint::<Secp256k1>::from_bytes(compressed_pub).unwrap()).unwrap();

    let signature = Signature::try_from(signature).unwrap();

    verifing_key.verify(message, &signature).is_ok()
}