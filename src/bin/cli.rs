use rudis::data_frame::{DataFrame, MAX_FRAME_SIZE};
use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    let mut server_stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    loop {
        let mut input_buffer = String::new();

        println!("Please enter Rudis command (currently GET KEY or SET KEY VAL or INCR KEY)");
        io::stdin().read_line(&mut input_buffer).unwrap();

        match DataFrame::from_string(input_buffer).to_byte_vec() {
            Ok(bytes) => {
                let _bytes_written = server_stream.write(&bytes).unwrap();
                server_stream.flush().unwrap();
                let mut received = [0;MAX_FRAME_SIZE];
                server_stream.read(&mut received).unwrap();
                println!("Received {:?}", DataFrame::from_bytes(received).unwrap());
            }
            Err(err) => {
                println!("Error parsing input {:?}", err);
                continue;
            }
        }
    }
}

