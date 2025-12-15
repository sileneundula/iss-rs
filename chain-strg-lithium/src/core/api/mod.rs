//! # API USAGE FOR CHAIN-STRG-LITHIUM
//! 

/// # StrgAPI
/// 
/// Basic STRG API
pub struct StrgAPI;

pub struct StrgWalletAPI {
    pub address: String,
    pub balance: u64,
    pub transactions: Vec<String>,
    
}

pub struct StrgNodeAPI;

pub struct StrgHTTPAPI;

pub struct StrgSmartContractAPI;