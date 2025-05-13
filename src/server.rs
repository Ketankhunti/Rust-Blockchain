use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::{block::Block, blockchain::Blockchain};

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:6000").await.unwrap();
    let mut blockchain = Blockchain::new();
    println!("🚀 Async server running on 6000");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buffer = [0; 1024];
        let n = tokio::time::timeout(std::time::Duration::from_secs(5), socket.read(&mut buffer)).await.unwrap().unwrap();

        let buffer_str = String::from_utf8_lossy(&buffer[..n]);
        let received_block: Block = serde_json::from_str(&buffer_str).unwrap();

       // println!("📩 Received block: {:?}", received_block);

        let is_valid = {
            let last = blockchain.blocks.last().unwrap();
           // println!("last: {:?}", last);
            received_block.previous_hash == last.hash &&
            received_block.hash == Block::calculate_hash(
                received_block.index,
                received_block.timestamp,
                received_block.data.clone(),
                received_block.previous_hash.clone()
            )
        };

        if is_valid {
            println!("✅ Block valid. Appending to blockchain.");
            // let index = blockchain.blocks.last().unwrap().index + 1;
            let block = Block::new(
                received_block.index,  // for now, will be chnaged later
                received_block.timestamp,
                received_block.data.clone(),
                blockchain.blocks.last().unwrap().hash.clone(),
            );
            blockchain.add_block(block);

            println!("Blockchain: {:#?}", blockchain);

            let response = serde_json::to_string(&received_block).unwrap();
            socket.write_all(response.as_bytes()).await.unwrap();
        } else {
            println!("❌ Block invalid. Rejecting.");
            socket.write_all(b"Invalid block").await.unwrap();
        }
    }
}
