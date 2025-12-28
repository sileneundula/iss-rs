//! # Immutable Storage Solution Standard (Rust)
//! 
//! [X] Framework
//!     - [X] Config
//!         - [ ] Consensus
//!             - [ ] Proof of Work
//!             - [ ] Proof of Stake
//!             - [ ] Delegated Proof of Stake
//!             - [ ] Delegated Proof of Work (DPOW5)
//!             - [ ] Proof of Authority
//!         - [X] Threshold
//!             - [X] Piece (256kb) (default: 0xffffffc000000000u64)
//!             - [ ] MiniPiece
//!             - [ ] TinyPiece
//!         - [ ] Timestamping
//!             - [ ] Proof-of-History
//!         - [ ] Networking
//!             - [ ] libp2p
//!                 - [ ] standard
//!                     - [ ] KAD
//!                     - [ ] MDNS
//!                     - [ ] Gossip
//!                     - [ ] PubSub


pub mod traits;


pub struct ISSFramework {
    config: Config,
}

pub struct Config {
    consensus: ConsensusMechanism, // Consensus Algorithm Chosen
    
    piece_threshold: Threshold, // Threshold for PoW
    storage: Storage,
}

pub enum ConsensusMechanism {
    ProofOfWork,
}

pub struct Threshold(u64);

impl Default for Threshold {
    fn default() -> Self {
        Threshold(0xffffffc000000000u64)
    }
}

pub enum Storage {
    strgu4, // 0-F
    strgu8, // 00-FF (256)
    strgu16, // 0000-FFFF
    strgu32, // ..
}

impl Default for Storage {
    fn default() -> Self {
        Self::strgu8
    }
}