use std::io::prelude::*;
use std::error::Error;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc::{channel, Sender};

pub struct RudisServer {
    client_addr: SocketAddr,
    server_addr: SocketAddr,
    client_tx: Sender<String>,
    server_tx: Sender<String>,
}
impl RudisServer {
    pub fn new(args: Vec<String>, client_tx: Sender<String>, server_tx: Sender<String>) -> Result<RudisServer, Box<dyn Error>> {
            let client_addr = args[1].parse::<SocketAddr>()?;
            let mut node_addr =  args[1].parse::<SocketAddr>()?;
            node_addr.set_port(client_addr.port() + 10000);
            Ok(RudisServer {
                client_addr: client_addr,
                server_addr: node_addr,
                client_tx,
                server_tx,
            })
        }

        pub fn listen(&self) -> Result<(), Box<dyn Error>> {
            let tcp_listener = TcpListener::bind(self.client_addr.to_string())?;
            for stream in tcp_listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer)?;
                println!("Request: {}", buffer);
            }
            Ok(())
        }
}
