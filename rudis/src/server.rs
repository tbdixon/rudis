use std::io::prelude::*;
use std::error::Error;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub struct RudisServer {
    client_addr: SocketAddr,
    server_addr: SocketAddr,
}
impl RudisServer {
    pub fn new(args: Vec<String>) -> Result<RudisServer, Box<dyn Error>> {
            let client_addr = args[1].parse::<SocketAddr>()?;
            let mut node_addr =  args[1].parse::<SocketAddr>()?;
            node_addr.set_port(client_addr.port() + 10000);
            Ok(RudisServer {
                client_addr: client_addr,
                server_addr: node_addr,
            })
        }

        pub fn listen(&self, client_tx: Sender<String>, server_tx: Sender<String>) -> Result<(), Box<dyn Error>> {
            let tcp_listener = TcpListener::bind(self.client_addr.to_string())?;
            thread::spawn(move || {
                for stream in tcp_listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer).unwrap();
                client_tx.send(buffer).unwrap();
            }
            });
            Ok(())
        }
}
