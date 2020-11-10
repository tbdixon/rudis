use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::iter;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
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
    pub client_socket_addr: SocketAddr,
    node_socket_addr: SocketAddr,
    client_tcp_listener: TcpListener,
    node_tcp_listener: TcpListener,
    node_type: NodeType,
    replicas: Vec<String>,
    buckets: Vec<u16>,
    cluster_nodes: HashMap<NodeID, RudisNode>,
    cluster_buckets: HashMap<u16, NodeID>,
}

impl RudisNode {
    pub fn new(args: Vec<String>) -> Result<RudisNode, Box<dyn Error>> {
        let client_socket_addr = args[1].parse::<SocketAddr>()?;
        let mut node_socket_addr = args[1].parse::<SocketAddr>()?;
        node_socket_addr.set_port(client_socket_addr.port() + 10000);
        let client_tcp_listener = TcpListener::bind(client_socket_addr.to_string())?;
        let node_tcp_listener = TcpListener::bind(node_socket_addr.to_string())?;
        let mut node = RudisNode {
            id: String::new(),
            client_socket_addr: client_socket_addr,
            node_socket_addr: node_socket_addr,
            client_tcp_listener: client_tcp_listener,
            node_tcp_listener: node_tcp_listener,
            node_type: NodeType::Solo,
            replicas: Vec::new(),
            buckets: Vec::new(),
            cluster_nodes: HashMap::new(),
            cluster_buckets: HashMap::new(),
        };
        node.set_node_id();
        Ok(node)
    }

    pub fn listen(&self) {
        thread::spawn(|| {
            for stream in self.client_tcp_listener.incoming() {
                self.listen_loop(stream.unwrap()).unwrap();
            }
        });
        /*    thread::spawn(|| {
            for stream in self.node_tcp_listener.incoming() {
                self.listen_loop(stream.unwrap()).unwrap();
            }
        });*/
    }

    pub fn node_listen(&self) -> Result<(), Box<dyn Error>> {
        for stream in self.node_tcp_listener.incoming() {
            self.listen_loop(stream?)?;
        }
        Ok(())
    }

    fn listen_loop(&self, stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = String::new();
        self.parse_request(stream, &mut buffer)?;
        println!("Request: {}", buffer);
        self.process_request(&buffer);
        Ok(())
    }

    fn process_request(&self, request: &String) {}

    fn parse_request(
        &self,
        mut stream: TcpStream,
        buffer: &mut String,
    ) -> Result<(), Box<dyn Error>> {
        stream.read_to_string(buffer)?;
        Ok(())
    }

    fn set_node_id(&mut self) {
        let mut rng = thread_rng();
        self.id = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(32)
            .collect();
    }
}
