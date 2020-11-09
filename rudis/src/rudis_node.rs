use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::prelude::*;
use std::iter;
use std::net::{SocketAddr, TcpListener, TcpStream};
type NodeID = String;

#[derive(Debug)]
enum NodeType {
    Primary,
    Replicas,
    Solo,
}

#[derive(Debug)]
pub struct RudisNode {
    pub id: NodeID,
    pub socket_addr: SocketAddr,
    tcp_listener: TcpListener,
    node_type: NodeType,
    replicas: Vec<String>,
    buckets: Vec<u16>,
    cluster_nodes: HashMap<NodeID, RudisNode>,
    cluster_buckets: HashMap<u16, NodeID>,
}

impl RudisNode {
    pub fn new(args: Vec<String>) -> RudisNode {
        let socket_addr = args[1].parse::<SocketAddr>().unwrap();
        let mut node = RudisNode {
            id: String::new(),
            socket_addr: socket_addr,
            tcp_listener: TcpListener::bind(socket_addr.to_string()).unwrap(),
            node_type: NodeType::Solo,
            replicas: Vec::new(),
            buckets: Vec::new(),
            cluster_nodes: HashMap::new(),
            cluster_buckets: HashMap::new(),
        };
        node.set_node_id();
        node
    }

    pub fn listen(&self) {
        for stream in self.tcp_listener.incoming() {
            let stream = stream.unwrap();
            let mut buffer = [0; 512];
            self.parse_request(stream, &mut buffer);
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            // parse buffer here
        }
    }

    fn parse_request(&self, mut stream: TcpStream, buffer: &mut [u8]) {
        stream.read(buffer).unwrap();
    }

    fn set_node_id(&mut self) {
        let mut rng = thread_rng();
        self.id = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(32)
            .collect();
    }
}
