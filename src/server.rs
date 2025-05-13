use std::{io::{Read, Write}, net::TcpListener};

use serde_json::from_str;

use crate::{block::Block, blockchain::Blockchain};
pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    let mut blockchain = Blockchain::new();
    println!("Server is running on port 6000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let buffer_str = String::from_utf8_lossy(&buffer[..bytes_read]);

        let received_block: Block = from_str(&buffer_str).unwrap();
        println!("Received block: {:#?}", received_block);

        let is_valid = {
            let last_block = blockchain.blocks.last().unwrap();
            received_block.previous_hash == last_block.hash &&
            received_block.hash == Block::calculate_hash(
                received_block.index,
                received_block.timestamp,
                received_block.data.clone(),
                received_block.previous_hash.clone()
            )
        };

        if is_valid {

            let last_block = blockchain.blocks.last().unwrap();

            let new_block = Block::new(
                last_block.index + 1,
                received_block.timestamp,
                received_block.data.clone(),
                last_block.hash.clone()
            );

            println!("✅ Valid block received! Adding to chain...");
            blockchain.add_block(new_block);
            let response: String = serde_json::to_string(&received_block).unwrap();
            stream.write_all(response.as_bytes()).unwrap();
          

        } else {
            println!("❌ Invalid block received! Rejecting...");
            let response = "Invalid block";
            println!("Response: {}", response);
            println!("last block: {:#?}", blockchain.blocks.last().unwrap());
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        println!("Blockchain length: {}", blockchain.blocks.len());
    }
}
