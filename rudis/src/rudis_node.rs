use std::sync::mpsc::Receiver;
use std::thread;

#[derive(Debug)]
enum NodeType {
    Primary,
    Replica,
}

#[derive(Debug)]
pub struct RudisNode {
    node_type: NodeType,
    replicas: Vec<String>,
}

impl RudisNode {
    pub fn new() -> RudisNode {
        RudisNode {
            node_type: NodeType::Primary,
            replicas: Vec::new(),
        }
    }

    pub fn make_replica(&mut self) {
        self.node_type = NodeType::Replica;
    }

    pub fn process(
        &self,
        client_rx: Receiver<String>,
        server_rx: Receiver<String>,
    ) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
        let ct = thread::spawn(|| {
            for msg in client_rx {
                println!("Client message received: {}", msg);
            }
        });
        let st = thread::spawn(|| {
            for msg in server_rx {
                println!("Server message received: {}", msg);
            }
        });
        (ct, st)
    }
}
