use crate::Result;
use std::error::Error;
use std::fmt;

pub const MAX_FRAME_SIZE: usize = 1024;
const TERMINATOR: [u8; 2] = [b'\r', b'\n'];

#[macro_export]
macro_rules! respbuff {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp_buff = [0;MAX_FRAME_SIZE];
            let mut _idx = 0;
            $(
                tmp_buff[_idx] = $x;
                _idx += 1;
            )*
            tmp_buff
        }
    };
}

#[macro_export]
macro_rules! vecarr{
    ( $x:expr ) => {
        {
            let mut tmp_buff = [0;MAX_FRAME_SIZE];
            for (idx,v) in $x.iter().enumerate() {
                tmp_buff[idx] = *v
            }
           tmp_buff
        }
    };
}

#[macro_export]
macro_rules! dferr {
    ( $x:expr ) => {
            Err(Box::new(DataFrameError($x.to_string())))
    };
}


#[derive(Debug)]
struct DataFrameError(String);
impl fmt::Display for DataFrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataFrameError: {}", self.0)
    }
}

impl Error for DataFrameError {}

pub struct DataFrameBuffer {
    frame: [u8; MAX_FRAME_SIZE],
    curr_idx: usize,
}

impl DataFrameBuffer {
    pub fn from_bytes(buff: [u8; MAX_FRAME_SIZE]) -> DataFrameBuffer {
        DataFrameBuffer{ frame: buff, curr_idx: 0, }
    }
 
    pub fn at_line_end(&self) -> Result<bool> {
        if self.curr_idx >= MAX_FRAME_SIZE - 1 {
            dferr!("DataFrameBuffer overflow")
        } 
        else {
            Ok(self.frame[self.curr_idx] == TERMINATOR[0] && self.frame[self.curr_idx + 1] == TERMINATOR[1])
        }
    }

    pub fn advance(&mut self) -> Result<()> {
        if self.curr_idx < MAX_FRAME_SIZE - 1 { 
            self.curr_idx += 1;
            Ok(())
        }
        else {
            dferr!("DataFrameBuffer overflow")
        }
    }

    pub fn eat_line(&mut self) -> Result<String> {
        let line_start = self.curr_idx;
        while !self.at_line_end()? {
            self.advance()?;
        }
        if self.at_line_end()? {
            self.advance()?;
            self.advance()?;
            Ok(String::from_utf8_lossy(&self.frame[line_start..self.curr_idx - 2]).to_string())
        }
        else {
            dferr!(format!("Parsing stopped before EOL marker one {:?}", &self.frame[..]))
        }
    }

    pub fn eat_bulk_string(&mut self, num_bytes: usize) -> Result<String> {
        let line_start = self.curr_idx;
        self.curr_idx += num_bytes + 2;
        Ok(String::from_utf8_lossy(&self.frame[line_start..self.curr_idx - 2]).to_string())
    }
 }

#[derive(Debug, PartialEq)]
pub enum DataFrame {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<DataFrame>),
}

impl DataFrame {
    pub fn to_byte_vec(self) -> Result <Vec<u8>> {
        let mut buffer = Vec::new();
        let add_frame_string = |frame_string: String| {
            let mut buffer = frame_string.bytes().collect::<Vec<u8>>(); 
            buffer.push(b'\r');
            buffer.push(b'\n');
            buffer
        };
        match self {
            DataFrame::SimpleString(simple_string) => {
                buffer.push(b'+');
                buffer.append(&mut add_frame_string(simple_string));
            },
            DataFrame::Error(error) => {
                buffer.push(b'-');
                buffer.append(&mut add_frame_string(error));
            },
            DataFrame::Integer(integer) => {
                buffer.push(b':');
                buffer.append(&mut add_frame_string(integer.to_string()));
           },
           DataFrame::BulkString(bulk_string) => {
                buffer.push(b'$');
                let mut bulk_len = bulk_string.len().to_string().bytes().collect();
                buffer.append(&mut bulk_len);
                buffer.push(b'\r');
                buffer.push(b'\n');
                buffer.append(&mut add_frame_string(bulk_string));
           },
           DataFrame::Array(array) => {
                buffer.push(b'*');
                let mut array_len_bytes = array.len().to_string().bytes().collect();
                buffer.append(&mut array_len_bytes);
                buffer.push(b'\r');
                buffer.push(b'\n');
                for frame in array {
                    buffer.append(&mut frame.to_byte_vec()?);
                }
           },
        };
        Ok(buffer)
    }

    pub fn from_string(s: String) -> DataFrame {
        let s: Vec<&str> = s.trim_end().split(' ').collect();
        let mut data_frames: Vec<DataFrame> = Vec::new();
        for parameter in s.iter() {
            if parameter.chars().all(char::is_numeric) {
                data_frames.push(DataFrame::Integer(parameter.parse::<i64>().unwrap()));
            }
            else {
                data_frames.push(DataFrame::BulkString(parameter.to_string()));
            }
        }
        DataFrame::Array(data_frames)
    }

    pub fn from_bytes(b: [u8;MAX_FRAME_SIZE]) -> Result<DataFrame> {
        DataFrame::from_df_buffer(&mut DataFrameBuffer::from_bytes(b))
    }

    pub fn from_df_buffer(frame: &mut DataFrameBuffer) -> Result<DataFrame> {
        let identifier = frame.frame[frame.curr_idx];
        frame.advance()?;
        match identifier {
            b'+' => {
                Ok(DataFrame::SimpleString(frame.eat_line()?))
            },
            b'-' => {
                Ok(DataFrame::Error(frame.eat_line()?))
            }
            b':' => {
                Ok(DataFrame::Integer(frame.eat_line()?.parse::<i64>()?))
            }
            b'$' => {
                let num_bytes = frame.eat_line()?.parse::<usize>()?;
                Ok(DataFrame::BulkString(frame.eat_bulk_string(num_bytes)?))
            }
            b'*' => {
                let num_elems = frame.eat_line()?.parse::<i32>()?;
                let mut data_array = Vec::with_capacity(num_elems as usize);
                for _ in 0..num_elems {
                    data_array.push(DataFrame::from_df_buffer(frame)?);
                }
                Ok(DataFrame::Array(data_array))
            }
            _ => dferr!(format!("Unknown RESP Identifier {}", identifier))
        }
    }

    pub fn parse_command(&self) -> Result<&str> {
        match self {
            DataFrame::BulkString(command) => Ok(command),
            DataFrame::Error(s) | DataFrame::SimpleString(s) => dferr!(s),
            DataFrame::Array(_) | DataFrame::Integer(_) => dferr!("Invalid command frame received")
        }
    }

    pub fn parse_val(&self) -> Result<i64> {
        match self {
            DataFrame::Integer(val) => Ok(*val),
            DataFrame::Error(_) | DataFrame::SimpleString(_) | DataFrame::Array(_) | DataFrame::BulkString(_) => dferr!("Value must be an integer")
        }
    }

    pub fn parse_key(&self) -> Result<&str> {
        match self {
            DataFrame::BulkString(key) => Ok(key),
            DataFrame::Error(_) | DataFrame::SimpleString(_) | DataFrame::Array(_) | DataFrame::Integer(_) => dferr!("Key must be a string")
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_at_line_end() {
        let mut frame = DataFrameBuffer::from_bytes(respbuff!(b'H', b'E', b'L', b'L', b'O', b'\r', b'\n'));
        frame.curr_idx = 5;
        assert!(frame.at_line_end().unwrap());
        frame.curr_idx = 4;
        assert!(!frame.at_line_end().unwrap());
    }
    
    #[test]
    fn test_simple_string() {
        let byte_buffer = respbuff!(b'+', b'H', b'E', b'L', b'L', b'O', b'\r', b'\n');
        let data_frame = DataFrame::SimpleString(String::from("HELLO"));
        assert_eq!(DataFrame::from_bytes(byte_buffer).unwrap(), data_frame);
        assert_eq!(vecarr!(data_frame.to_byte_vec().unwrap())[..], byte_buffer[..]);
    }

    #[test]
    fn test_to_error() {
        let byte_buffer = respbuff!(b'-', b'E', b'R', b'R', b'O', b'R', b'\r', b'\n');
        let data_frame = DataFrame::Error(String::from("ERROR"));
        assert_eq!(DataFrame::from_bytes(byte_buffer).unwrap(), data_frame);
        assert_eq!(vecarr!(data_frame.to_byte_vec().unwrap())[..], byte_buffer[..]);
    }

    #[test]
    fn test_to_integer() {
        let byte_buffer = respbuff!(b':', b'4', b'2', b'0', b'0', b'0', b'\r', b'\n');
        let data_frame = DataFrame::Integer(42000);
        assert_eq!(DataFrame::from_bytes(byte_buffer).unwrap(), data_frame);
        assert_eq!(vecarr!(data_frame.to_byte_vec().unwrap())[..], byte_buffer[..]);
     }

    #[test]
    fn test_to_bulk_string() {
        let byte_buffer = respbuff!(b'$',b'5', b'\r', b'\n',  b'H', b'E', b'\r', b'\n', b'O', b'\r', b'\n');
        let data_frame = DataFrame::BulkString(String::from("HE\r\nO"));
        assert_eq!(DataFrame::from_bytes(byte_buffer).unwrap(), data_frame);
        assert_eq!(vecarr!(data_frame.to_byte_vec().unwrap())[..], byte_buffer[..]);
     }

    #[test]
    fn test_array() {
        let byte_buffer = respbuff!(b'*',b'3', b'\r', b'\n',b'+',  b'S', b'E', b'T', b'\r', b'\n', b'+', b'F',b'O', b'O', b'\r', b'\n', b':', b'4', b'2', b'\r', b'\n');
        let data_frame = DataFrame::Array(vec![DataFrame::SimpleString(String::from("SET")), DataFrame::SimpleString(String::from("FOO")), DataFrame::Integer(42)]);
        assert_eq!(DataFrame::from_bytes(byte_buffer).unwrap(), data_frame);
        assert_eq!(vecarr!(data_frame.to_byte_vec().unwrap())[..], byte_buffer[..]);
     }
}
