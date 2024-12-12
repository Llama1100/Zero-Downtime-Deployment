use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 8491
// Hash 7785
// Hash 3322
// Hash 7530
// Hash 9485
// Hash 8122
// Hash 7567
// Hash 6782
// Hash 2080
// Hash 6791
// Hash 4219
// Hash 8747
// Hash 7826
// Hash 9630
// Hash 2806
// Hash 4667
// Hash 5356
// Hash 2342
// Hash 7782
// Hash 9323
// Hash 6299
// Hash 6311
// Hash 7607
// Hash 3515
// Hash 2489
// Hash 3442
// Hash 3762
// Hash 1228
// Hash 8033
// Hash 3115
// Hash 5123
// Hash 4094
// Hash 9895
// Hash 2712
// Hash 1293
// Hash 9084
// Hash 9002
// Hash 8950
// Hash 1776
// Hash 9227
// Hash 2231
// Hash 4230
// Hash 4779
// Hash 4780
// Hash 6141
// Hash 9703
// Hash 4948
// Hash 5727
// Hash 1742
// Hash 1651
// Hash 3738
// Hash 3889
// Hash 3981
// Hash 9971
// Hash 6568
// Hash 5843
// Hash 3724
// Hash 6061
// Hash 2364
// Hash 4310
// Hash 9052
// Hash 9669
// Hash 7628
// Hash 2320
// Hash 9580
// Hash 8322
// Hash 8830
// Hash 2433
// Hash 2999
// Hash 9622
// Hash 2656
// Hash 2884
// Hash 8013
// Hash 2247
// Hash 1349
// Hash 9739
// Hash 5741
// Hash 4624
// Hash 9036
// Hash 6460
// Hash 1442
// Hash 6719
// Hash 3417
// Hash 3073
// Hash 6196