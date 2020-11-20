use rudis::data_frame::{DataFrame, MAX_FRAME_SIZE};
use std::net::TcpStream;
use std::thread;
use std::io::{Read, Write};

fn main() {
    let mut handles = Vec::new();
    let num_threads = 1;
    let target_num = 1000000;
    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut server_stream = TcpStream::connect("127.0.0.1:8000").unwrap();
            for idx in 0..target_num / num_threads {
                let input_buffer = "INCR FOO".to_string();
                let bytes =DataFrame::from_string(input_buffer).to_byte_vec().unwrap();
                let _bytes_written = server_stream.write(&bytes).unwrap();
                server_stream.flush().unwrap();
                let mut received = [0;MAX_FRAME_SIZE];
                server_stream.read(&mut received).unwrap();
                if idx % 10000 == 0{
                    println!("Thread {} Received {:?}", i, DataFrame::from_bytes(received).unwrap());
                }
            }
        });
        handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
}
