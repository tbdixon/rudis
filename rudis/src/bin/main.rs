use rudis::rudis_node::RudisNode;
use rudis::server::RudisServer;
use std::env;
use std::error::Error;
use std::sync::mpsc::channel;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (client_tx, client_rx) = channel();
    let (server_tx, server_rx) = channel();
    let node = RudisNode::new(client_rx, server_rx);
    let server = RudisServer::new(args)?;
    println!("Node and server created");
    server.listen(client_tx, server_tx)?;
    node.process();
    Ok(())
}
