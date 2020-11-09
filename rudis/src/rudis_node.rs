use std::collections::HashMap;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

type NodeID = String;
type IpAddr = String;

#[derive(Debug)]
enum NodeType {
    Primary,
    Replicas,
    Solo, 
}

#[derive(Debug)]
pub struct RudisNode {
    id: NodeID,
    ip_addr: IpAddr,
    node_type: NodeType,
    replicas: Vec<String>,
    buckets: Vec<u16>,
    cluster_nodes: HashMap<NodeID, RudisNode>,
    cluster_buckets: HashMap<u16, NodeID>,
}

impl RudisNode {
    pub fn new(ip_addr: IpAddr) -> RudisNode {
        let mut node = RudisNode {
            id: String::new(),
            ip_addr: ip_addr,
            node_type: NodeType::Solo,
            replicas: Vec::new(),
            buckets: Vec::new(),
            cluster_nodes: HashMap::new(),
            cluster_buckets: HashMap::new(),
        };
        node.set_node_id();
        node
    }
    fn set_node_id(&mut self) {
        let mut rng = thread_rng();
        self.id = iter::repeat(()).map(|()| rng.sample(Alphanumeric)).take(32).collect();
        println!("{} : {}", self.ip_addr, self.id);
    }
}
