use hex_literal::hex;
use sha2::{Digest, Sha256};

fn generate_salt() -> String {
    let mut bytes = [0u8; 4]; // TODO: constant
    rand::fill(&mut bytes);
    hex::encode(bytes)
}

fn main() {
    let salt = generate_salt();
    println!("{}", salt);
    let result = Sha256::digest(b"abcdad4163ee");
    println!("{}", hex::encode(result));
}
