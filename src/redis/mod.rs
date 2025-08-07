mod connection;
mod operations;
mod error;

pub use connection::{get_pool};
pub use operations::{get_value, set_value, with_connection};
pub use error::RedisPoolError;