/// 256kb Blob (256*1024) | 262,144 bytes
pub const PIECE_SIZE_IN_BYTES: usize = 262_144;

/// 16kb Blob (16*1024) | 16,384 bytes
pub const MINI_PIECE_SIZE_IN_BYTES: usize = 16_384;

/// 4kb (4*1024) | 4,096 bytes
pub const TINY_PIECE_SIZE_IN_BYTES: usize = 4_096;

use base58::{FromBase58,ToBase58,FromBase58Error};
use serde::{Serialize,Deserialize};
use serde_big_array::BigArray;


/// # Piece
/// 
/// One piece is equal to 256kb
#[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct Piece(#[serde(with = "BigArray")][u8;PIECE_SIZE_IN_BYTES]);

#[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct MiniPiece(#[serde(with = "BigArray")][u8;MINI_PIECE_SIZE_IN_BYTES]);

#[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct TinyPiece(#[serde(with = "BigArray")][u8;TINY_PIECE_SIZE_IN_BYTES]);


impl Piece {
    pub fn new() {

    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut output_bytes = [0u8;PIECE_SIZE_IN_BYTES];
        
        if bytes.len() == PIECE_SIZE_IN_BYTES {
            output_bytes.copy_from_slice(bytes);
            return Self(output_bytes)
        }
        else {
            panic!("Bytes do not match")
        }
    }


    pub fn to_base58(&self) -> String {
        return self.0.to_base58()
    }
    pub fn from_base58(s_bs58: &str) -> Result<Vec<u8>, FromBase58Error> {
        s_bs58.from_base58()
    }
    
    
    
    
    
    
    pub fn as_bytes(&self) -> &[u8] {
        return &self.0
    }
    pub fn to_vec(&self) -> Vec<u8> {
        return self.0.to_vec()
    }
    pub fn to_bytes(&self) -> [u8;PIECE_SIZE_IN_BYTES] {
        return self.0
    }
    pub fn piece_size() -> usize {
        return PIECE_SIZE_IN_BYTES
    }
}