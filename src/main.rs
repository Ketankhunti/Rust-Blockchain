mod block;
mod blockchain;
mod server;
mod client;

use blockchain::Blockchain;
use server::run_server;
use std::thread;
use client::run_client;
fn main() {
   
   let _ = Blockchain::new();

    thread::spawn(|| {
        run_client();
    });


   run_server();
}
