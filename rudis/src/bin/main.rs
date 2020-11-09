use rudis::rudis_node::{IpAddr, NodeType, RudisNode};

fn main() {
    let ip_addr = "127.0.0.1";
    let ports = vec![8080, 8081, 8082, 8083, 8084, 8085];
    let ip_addr = IpAddr { ip: ip_addr.to_string(), port: 8080 };
    let n = RudisNode { id: String::from("abcd"), addr: ip_addr, node_type: NodeType::Master };
    println!("{:?}", n);
}
