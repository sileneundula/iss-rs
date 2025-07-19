//! # MiniProofofWork (Nano-Style)
//! 
//! Uses BLAKE2B to generate a valid nonce given an input such as a transaction hash, or in this case, the piece data bytes.

use libslug::slugcrypt::api::SlugCSPRNGAPI;


pub const PIECE_THRESHOLD: u64 = 0xffffffc000000000;
pub const MINI_PIECE_THRESHOLD: u64 = 0xffff000000000000;

pub struct MiniPoW;

impl MiniPoW {
    pub fn new(bytes: &[u8], threshold: u64) -> u64 {
        return blake2b_pow::mine(bytes, threshold)
    }
    pub fn verify(bytes: &[u8], nonce: u64, threshold: u64) -> bool {
        return blake2b_pow::verify_nonce(bytes, threshold, nonce)
    }
}

type minipow_nonce = u64;