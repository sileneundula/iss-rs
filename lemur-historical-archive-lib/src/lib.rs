//! # Lemur (Historical Data)
//! 
//! - [ ] TODO
//!     - [ ] LemurAlgorithm
//!         - [ ] An algorithm for storing data efficiently into pieces that allows sharing shards of data.
//!         - [ ] An algorithm that allows for quick and efficient searching for pieces using BLAKE3.
//!     - [X] LemurFile
//!         - [X] Structs 
//!         

/// # Lemur
/// 
/// 
pub struct Lemur;

/// Output File
pub mod output;

/// Algorithms
pub mod algorithms;

pub struct LemurHistoricalData {
    id: u64,
    registrant: String,
}

pub struct LemurRegistar {
    pub id: u32, // Registered ID
    pub init_public_key: String, // Initial Public Key
    pub address: String, // 64-byte Address In Base32

}