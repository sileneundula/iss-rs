//! # Modern Address Scheme
//! 
//! Based off of multiaddress scheme concepts
//! 
//! ## Features
//! 
//! - [X] Modern Addressing Scheme
//!     - [X] Use UTF-8
//!         - [ ] Add ASCII Option
//!     - [ ] Support for multiple protocols
//!     - [ ] Conversion between MultiAddr and ModernAddressingScheme


/// # Modern Address Scheme (MAS)
/// 
/// 
pub struct ModernAddressScheme {
    pub pieces: Vec<String>,
    pub parser: ModernAddressSchemeParser,
    pub checksum: Option<String>, // 6-byte checksum in blake2s
}


pub trait ModernAddressSchemeTrait {

}


impl ModernAddressScheme {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        if s.as_ref().contains("/") {
            let x: std::str::Split<'_, &str> = s.as_ref().split("/");
            let mut pieces: Vec<String> = vec![];

            for i in x {
                pieces.push(i.to_owned())
            }
            return Self {
                pieces: pieces,
                parser: ModernAddressSchemeParser::Standard,
                checksum: None,
            }
        }
        else {
            let mut pieces: Vec<String> = vec![];
            pieces.push(s.as_ref().to_owned());
            return Self {
                pieces: pieces,
                parser: ModernAddressSchemeParser::Standard,
                checksum: None,
            }
        }
    }
}

pub enum ModernAddressSchemeParser {
    Standard,
}

pub struct PeerAddress {
    pub address: String,
}