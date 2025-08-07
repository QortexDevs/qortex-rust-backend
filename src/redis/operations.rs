use crate::redis::connection::get_pool;
use crate::redis::error::RedisPoolError;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis::Commands;

pub fn with_connection<F, T>(operation: F) -> Result<T, RedisPoolError>
where
    F: FnOnce(&mut PooledConnection<RedisConnectionManager>) -> Result<T, RedisPoolError>,
{
    let pool = get_pool();
    let mut con = pool.get()?;
    operation(&mut con)
}

pub fn get_value(key: &str) -> Result<Option<String>, RedisPoolError> {
    with_connection(|con| {
        match con.get(key) {
            Ok(value) => Ok(Some(value)), // Key exists → Some(value)
            Err(e) if e.kind() == r2d2_redis::redis::ErrorKind::TypeError => Ok(None), // Key doesn't exist → None
            Err(e) => Err(RedisPoolError::from(e)), // Other errors (e.g., connection issues)
        }
    })
}

pub fn set_value(key: &str, value: &str) -> Result<(), RedisPoolError> {
    with_connection(|con| con.set(key, value).map_err(RedisPoolError::from))
}
