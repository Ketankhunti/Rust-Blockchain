use crate::block::Block;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, 0, "Genesis Block".to_string(), "0".to_string());
        Blockchain { blocks: vec![genesis_block] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    
}

