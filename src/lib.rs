//! # ImmutableStorageSolution (ISS)
//! 
//! ImmutableStorageSolution is a blockchain-based storage solution that uses pieces to store data. This is similar to IPFS, except that these pieces are permenant.
//! 
//! ## TODO
//! 
//! [] Blockchain
//!     []
//! 
//! [] Pieces Storage
//!     [X] Piece (256kb)
//!         [X] To/From Base58
//!         [X] To/From Bytes
//!     [] Mini Piece (16kb)
//!     [] Tiny Piece (4kb)
//! 
//!     [] Blob (Base58/Base64)
//! 


/// Core Components
pub mod core;

/// Config
pub mod config;

/// App
pub mod app;


pub struct ImmutableStorageSolution;

pub struct ImmutableStorageSolutionConfig {

}
