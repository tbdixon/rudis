pub mod data_frame;
pub mod db;
pub mod server;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
