use crate::core::minipow::PIECE_THRESHOLD;
use crate::prelude::*;
use crate::core::xrypto::Blake3Hasher;
use crate::core::xrypto::SlugDigester;
use crate::core::xrypto::SlugDigest;
use crate::core::minipow::MiniPoW;

pub struct ISSBlock {
    id: u64,
    prev_hash: String, // previous hash of the last block
    pieces: Vec<Blob>, // up to 16 = 4mb
    b3_address: Vec<String>, // BLAKE3 Digests
}

pub struct Blob {
    pub piece: Piece,
    pub blake3: String,
    
    // Extra Security/Integrity
    pub blake2b: String,
    pub sha3: String,
    pub sha2: String,

    pub nonce: Option<u64>,
}

impl Blob {
    pub fn from_piece(piece: Piece) -> Self {
        let bytes = piece.as_bytes();
        let blake3 = hash_input_b3(bytes);
        let (blake2b_64, sha2_256, sha3_224) = hashes(bytes);

        Self {
            piece: piece,
            blake3: blake3.clone(),
            blake2b: blake2b_64,
            sha3: sha3_224,
            sha2: sha2_256,
            nonce: Some(MiniPoW::new(blake3.as_bytes(), PIECE_THRESHOLD)),
        }

    }
    pub fn blake3(&self) -> String {
        return self.blake3.clone()
    }
    pub fn as_blake3(&self) -> &str {
        return &self.blake3
    }
}




fn hash_input_b3(bytes: &[u8]) -> String {
    let hasher = Blake3Hasher::new();
    let output = hasher.update(bytes);
    return SlugDigester::from_bytes(&output).unwrap().digest().to_string()
}

fn hashes(bytes: &[u8]) -> (String,String,String) {
    let blake2b_64 = SlugDigest::blake2b(64, bytes).digest().to_string();
    let sha2 = SlugDigest::sha2(256, bytes).digest().to_string();
    let sha3 = SlugDigest::sha3(224, bytes).digest().to_string();

    return (blake2b_64,sha2,sha3)
}