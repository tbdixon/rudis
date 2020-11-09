use rudis::rudis_node::RudisNode;

fn main() {
    let ips = vec!["127.0.0.1:8080", "127.0.0.1:8081", "127.0.0.1:8082", "127.0.0.1:8083", "127.0.0.1:8084", "127.0.0.1:8085"];
    let node = RudisNode::new(ips[0].to_string());
    println!("{:?}", node);
}
