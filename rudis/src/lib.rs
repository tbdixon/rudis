pub mod server;
pub mod db;
pub mod data_frame;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
