//! # ImmutableStorageSolution
//! 
//! The ImmutableStorageSolution is a simple but novel method of storing immutable data on the blockchain without having to pay a form of currency.
//! 
//! In this system, Proof-of-Work (nano-style) is performed to deter spam and let only a certain amount of data be uploaded.
//! 
//! This data is then sorted by u8/u16 depending on the chain.
//! 
//! ## TODO
//! 
//! [] Interoperability
//! 
//! 

pub struct ImmutableStorageSolutionApp;

impl ImmutableStorageSolutionApp {
    pub fn new() -> Self {
        return Self
    }
}

pub struct ISSChainRegistration {
    public_facing_id: u128, // id for all chains
    
    unique_name: String,
    digest_id: String, // 12-bytes (appended to unique_name)
    namespace: String,
}