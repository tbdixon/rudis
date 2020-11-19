use crate::db::RudisDb;
use crate::data_frame::{self, DataFrameBuffer};
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn handle_connection(mut stream: TcpStream, _: RudisDb) {
    let mut buffer = [0; data_frame::MAX_FRAME_SIZE];
    loop {
        match stream.read(&mut buffer) {
            Ok(num_bytes) => {
                if num_bytes > 0 {
                    let data_frame = data_frame::dataframe_from_bytes(&mut DataFrameBuffer::new(buffer));
                    println!("Dataframe received: {:?}", data_frame);
                }
                else{
                    println!("Connection closed");
                    break;
                }
            },
            Err(err) => {
                println!("Error with remote command: {:?}", err);
                break;
            }
        }
    }
}

pub fn run(listener: TcpListener) -> Result<(), Box<dyn Error>> {
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
