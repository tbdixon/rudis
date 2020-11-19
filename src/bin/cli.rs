use rudis::data_frame;
use rudis::data_frame::DataFrame::{self, *};
use std::io::{self, Write};
use std::net::TcpStream;

fn main() {
    let mut server_stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    loop {
        let mut input_buffer = String::new();
        println!("Please enter Rudis command");
        io::stdin().read_line(&mut input_buffer).unwrap();
        let data_frame = parse_user_input(input_buffer);
        let _bytes_written = server_stream.write(&data_frame::bytes_from_dataframe(data_frame).unwrap()).unwrap();
        server_stream.flush().unwrap();
    }
}

fn parse_user_input(user_input: String) -> DataFrame {
    let user_input: Vec<&str> = user_input.trim_end().split(' ').collect();
    let mut data_frames: Vec<DataFrame> = Vec::new();
    for parameter in user_input.iter() {
        if parameter.chars().all(char::is_numeric) {
            data_frames.push(Integer(parameter.parse::<i64>().unwrap()));
        }
        else {
            data_frames.push(BulkString(parameter.to_string()));
        }
    }
    Array(data_frames)
}
