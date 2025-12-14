pub trait ConsensusMechanism {
    /// Returns Consensus Algorthim
    /// 
    /// - Proof-of-Work-Lithium (PoW)
    /// - Proof-of-Stake-Lithium (PoS)
    /// - Delegated-Proof-of-Stake (DPOS)
    /// - Delegated-Proof-of-Work-0x20CB (DPOW-0x20CB)
    fn algorithm(&self) -> String;
    /// Returns Configuration
    fn config(&self) -> ConsensusConfig;
    /// Can Integrate Into Blockchain
    fn integration(&self) -> bool;
}

pub struct ConsensusConfig {

}
