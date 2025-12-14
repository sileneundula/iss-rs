pub struct ConsensusAlgorithm {
    pub Algorithm: 
}

pub trait IsConsensusAlgorithm {

}

pub struct Consensus_PoW {
    target_difficulty: usize, // Number of zeroes
    interval: usize,

}

pub struct PoW_BTC_Algorithm;

impl PoW_BTC_Algorithm {
    pub fn new(difficulty: usize) {
        
    }
}

impl IsConsensusAlgorithm for Consensus_PoW {

}