use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::db::RudisDb;

pub fn handle_connection(mut stream: TcpStream, db: RudisDb) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    match &buffer[0..4] {
        "PUT " => db.put(buffer),
        "GET " => db.get(buffer),
        _ => println!("UNKNOWN COMMAND!")
    }
}

pub fn run(listener: TcpListener) -> Result<(), Box< dyn Error>>{
    let db = RudisDb::new(); 
    for stream in listener.incoming() {
        let counter = db.clone();
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream, counter); 
        });
    }
    Ok(())
}
