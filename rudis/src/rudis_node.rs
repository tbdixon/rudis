use std::sync::mpsc::Receiver;

#[derive(Debug)]
enum NodeType {
    Primary,
    Replica,
}

#[derive(Debug)]
pub struct RudisNode {
    node_type: NodeType,
    client_rx: Receiver<String>,
    server_rx: Receiver<String>,
    replicas: Vec<String>,
}

impl RudisNode {
    pub fn new(client_rx: Receiver<String>, server_rx: Receiver<String>) -> RudisNode {
        RudisNode {
            node_type: NodeType::Primary,
            client_rx,
            server_rx,
            replicas: Vec::new(),
        }
    }

    pub fn make_replica(&mut self) {
        self.node_type = NodeType::Replica;
    }

    pub fn process(self) {
        for msg in self.client_rx {
            println!("Message received: {}", msg);
        }
    }
}

