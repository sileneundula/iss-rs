//! # STRG Lithium Chain: An Immutable Storage Chain
//! 
//! A Decentralized Storage Network Chain. Use a DAG.
//! 
//! DAG-Based

pub struct StrgEcosystem {
    id: u64,

}

pub mod core;

impl StrgEcosystem {
    pub fn new(id: u64) -> Self {
        Self {
            id
        }
    }
}