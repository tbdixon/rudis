use rudis::server;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() -> Result<(), Box<dyn Error>> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);
    let client_tcp_listener = TcpListener::bind(socket)?;
    println!("Server starting to listen at {}", socket);
    server::run(client_tcp_listener)?;
    Ok(())
}
