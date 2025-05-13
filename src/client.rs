use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::to_string;

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::block::Block;

pub fn run_client() {
    let previous_hash = "0794b58b1cf3c7a485013f98b3c35fc4cc271889df22b99d0ed152defaf97d1a".to_string();
    let block = Block::new(
        1,
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "Test block from client".to_string(),
        previous_hash,
    );

    let serialized = to_string(&block).unwrap();
    let mut stream = TcpStream::connect("127.0.0.1:6000").unwrap();
    stream.write_all(serialized.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("📦 Block sent to server");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("📦 Response from server: {}", response);
}

