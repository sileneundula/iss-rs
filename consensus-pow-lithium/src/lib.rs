//! # POW Lithium
//! 
//! Highest-Difficulty: 0x1d00ffff | 0x00000000FFFF0000000000000000000000000000000000000000000000000000
//! 
//! ## How To Find Block Generation Time Based On Difficulty
//! 
//! time = difficulty * 2**32 / hashrate

pub const MECHANISM_POW: &str = "CONSENSUS_POW";
pub const MECHANISM_POW_LITHIUM: &str = "CONSENSUS_POW_LITHIUM";

use bitcoin_u256::u256;
use digest::Digest;




pub struct Mechanism<Hasher: Digest> {
    // label
    label: String,
    
    
    // difficulty
    initial_target_difficulty: u64,
    // Block Interval For Block Difficult Update
    block_interval: u32,
    current_difficulty: u64, // not to exceed 0x1d00ffff

    // Hasher
    hash: Hasher
}

impl Mechanism {
    
}

impl<Hasher: Digest> Mechanism {
    pub fn work<T: AsRef<[u8]>>(&self, input: T) {
        self.hash
    }

}