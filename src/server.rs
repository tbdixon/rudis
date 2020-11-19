use crate::db::RudisDb;
use crate::data_frame::{MAX_FRAME_SIZE, DataFrame};
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn handle_connection(mut stream: TcpStream, db: RudisDb) {
    let mut buffer = [0; MAX_FRAME_SIZE];
    loop {
        match stream.read(&mut buffer) {
            Ok(num_bytes) if num_bytes > 0 => {
                let resp = match DataFrame::from_bytes(buffer) {
                    Ok(DataFrame::Array(data_frame)) => {
                        let command = data_frame[0].parse_command().unwrap();
                        match command {
                            "SET" => {
                                let (key, val) = (data_frame[1].parse_key().unwrap(), data_frame[2].parse_val().unwrap());
                                let mut db = db.db.lock().unwrap();
                                db.insert(key.to_string(),val);
                                DataFrame::Integer(val)
                            },
                            "GET" => {
                                let key = data_frame[1].parse_key().unwrap();
                                let db = db.db.lock().unwrap();
                                match db.get(key) {
                                    Some(val) => DataFrame::Integer(*val),
                                    None => DataFrame::Error(format!("Key {} not found", key))
                                }
                            },
                            "INCR" => {
                                let key = data_frame[1].parse_key().unwrap();
                                let mut db = db.db.lock().unwrap();
                                *db.entry(key.to_string()).or_insert(0) += 1;
                                DataFrame::Integer(*db.get(key).unwrap()) 
                           },
                            _ => {
                                DataFrame::Error(format!("Invalid command: {}", command))
                           }
                        }
                    },
                    _ => {
                        DataFrame::Error("Invalid data frame type".to_string())
                    }
                };
                let _bytes_written = stream.write(&resp.to_byte_vec().unwrap()).unwrap();
                stream.flush().unwrap();
            },
            Ok(_) => {
                println!("Remote connection closed");
                break;
            },
            Err(err) => {
                println!("Error parsing remote command: {}", err);
                break;
            }
        };
        }
    }

pub fn run(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    let db = RudisDb::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let counter = db.clone();
        thread::spawn(move || {
            handle_connection(stream, counter);
        });
    }
    Ok(())
}
