use rudis::rudis_node::RudisNode;
use rudis::server::RudisServer;
use std::env;
use std::error::Error;
use std::sync::mpsc::channel;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (client_tx, client_rx) = channel();
    let (server_tx, server_rx) = channel();
    let node = RudisNode::new();
    let server = RudisServer::new(args)?;
    println!("Node and server created");
    let (client_tx_thread, server_tx_thread) = server.listen(client_tx, server_tx)?;
    let (client_rx_thread, server_rx_thread) = node.process(client_rx, server_rx);
    client_tx_thread.join().unwrap();
    server_tx_thread.join().unwrap();
    client_rx_thread.join().unwrap();
    server_rx_thread.join().unwrap();
    Ok(())
}
