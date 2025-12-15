//! # DAG-Block
//! 
//! ## Features
//! 
//! - [ ] Sorted by {u4,u8,u16}
//! 
//! 

pub struct InitialBlock {
    PieceGathering: PieceGathering,
    
    address: String,
    public_key: String,
    description: String,

    signature: String,
}

pub struct Block {
    id: u64,
    prev_hash: String,
    
    // Address
    address: String,


}

pub enum PieceGathering {
    _u4,
    _u8,
    _u16,
}