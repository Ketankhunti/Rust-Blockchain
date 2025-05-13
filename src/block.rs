use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Self {
        let hash = Block::calculate_hash(index, timestamp, data.clone(), previous_hash.clone()  );
        Block { index, timestamp, data, previous_hash, hash }
    }   

    pub fn calculate_hash(index: u32, timestamp: u64, data: String, previous_hash: String) -> String {
        let input = format!("{}:{}:{}:{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        hex::encode(hash)
    }
}