use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use hex;

pub fn generate_keypair_and_address() -> (SecretKey, PublicKey, Vec<u8>) {
    // Initialize the secp256k1 context
    let secp = Secp256k1::new();

    // Generate a random private key
    let (private_key, public_key) = secp.generate_keypair(&mut OsRng);
    
    // Derive the Ethereum address from the public key
    let public_key_bytes = public_key.serialize_uncompressed();
    let hash = Keccak256::digest(&public_key_bytes[1..]); // Skip the 0x04 prefix
    let address = &hash[hash.len() - 20..]; // Take the last 20 bytes

    (private_key, public_key, address.to_vec())
}

fn main() {
  let (private_key, public_key, address) = generate_keypair_and_address();
  println!("Private Key: 0x{}", hex::encode(private_key.secret_bytes()));
  println!("Uncompressed Public Key: 0x{}", hex::encode(public_key.serialize_uncompressed()));
  println!("Address: 0x{}", hex::encode(address));
}