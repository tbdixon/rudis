use std::error::Error;
use crate::Result;
use std::fmt;

#[derive(Debug)]
struct DataFrameError(String);

impl fmt::Display for DataFrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataFrameError: {}", self.0)
    }
}

impl Error for DataFrameError {}

pub enum DataFrame {
    SimpleString(String),
    Error(String),
    Integer(i32),
    BulkString(String),
    Array(Vec<DataFrame>),
    None,
}

impl DataFrame {
    const TERMINATOR: [u8;2] = [b'\r', b'\n'];
    const MAX_FRAME_SIZE: usize = 1024;
    
    pub fn is_at_end(frame: &[u8;DataFrame::MAX_FRAME_SIZE], idx: usize) -> bool {
        frame[idx] == DataFrame::TERMINATOR[0] && frame[idx+1] == DataFrame::TERMINATOR[1]
    }

    pub fn parse(frame: [u8;DataFrame::MAX_FRAME_SIZE]) -> Result<DataFrame> {
        let frame_identifier = frame[0];
        match frame_identifier {
            b'+' => {
                let mut frame_idx = 1;
                while frame_idx < DataFrame::MAX_FRAME_SIZE - 2 && !DataFrame::is_at_end(&frame, frame_idx){
                    frame_idx += 1;
                }
                if DataFrame::is_at_end(&frame, frame_idx) {
                    return Ok(DataFrame::SimpleString(String::from_utf8_lossy(&frame[1..frame_idx-1]).to_string()))
                }
                else{
                    return Err(Box::new(DataFrameError("Error!".to_string())))
                }
            }
            _ => {
                return Err(Box::new(DataFrameError("Error!".to_string())))
            }
        }
    }
}
