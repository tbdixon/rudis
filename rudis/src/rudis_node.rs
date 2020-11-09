use std::collections::HashMap;
type NodeID = String;

#[derive(Debug)]
pub struct IpAddr {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug)]
pub enum NodeType {
    Master,
    Slave,
}

#[derive(Debug)]
pub struct RudisNode {
    pub id: NodeID,
    pub addr: IpAddr,
    pub node_type: NodeType,
/*    slaves: Vec<String>,
    buckets: Vec<u16>,
    cluster_nodes: HashMap<NodeID, RudisNode>,
    cluster_buckets: HashMap<u16, NodeID>,*/
}
