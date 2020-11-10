use rudis::rudis_node::RudisNode;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let node = RudisNode::new(args)?;
    println!(
        "Created a new node with address {} and ID {}. Listening for commands",
        node.client_socket_addr, node.id
    );
    node.listen();
    Ok(())
}
