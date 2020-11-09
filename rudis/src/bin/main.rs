use rudis::rudis_node::RudisNode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let node = RudisNode::new(args);
    println!(
        "Created a new node with address {} and ID {}. Listening for commands",
        node.socket_addr, node.id
    );
    node.listen();
}
