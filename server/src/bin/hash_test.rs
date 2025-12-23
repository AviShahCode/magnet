use hex_literal::hex;
use sha2::{Digest, Sha256};
fn main() {
    let result = Sha256::digest(b"user1234");
    print!("{}", hex::encode(result));
}
