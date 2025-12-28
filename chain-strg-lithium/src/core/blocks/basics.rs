//! A Block Should Be Up To 32MB in Size. 256kb x 4 x 32. Also include a 16kb piece for metadata.
//! 
//! 4 x 32 = 128 pieces per block of 256kb


pub struct Block {
    id: u64,
    prev_hash: String,
}

pub trait StrgBlock {

}