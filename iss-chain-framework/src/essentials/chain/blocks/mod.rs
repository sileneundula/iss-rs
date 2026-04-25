//! # Immutable Storage Solution: Framework For Immutable, Censorship-Resistant, Persistent Storage
//! 
//! ## Outline
//! 
//! - [ ] Chain
//!     - [ ] IssInitBlock
//!         - [ ] Public Key Owner Block
//!         - [ ] Interoperability Block
//!     - [ ] IssBlock: Contains the data (pieces)
//!         - [ ] Sizes {u4,u8,u16}
//!     - [ ] IssIndex: An Indexing Block

use fixedstr::str192;



pub struct IssInitBlock {

}

pub struct IssBlock {
    id: u64,
    prev_hash: str192,
    
    data: u8,
}