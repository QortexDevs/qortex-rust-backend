use r2d2::Error as R2D2Error;
use r2d2_redis::redis::RedisError;
use std::fmt;

#[derive(Debug)]
pub enum RedisPoolError {
    Redis(RedisError),
    Pool(R2D2Error),
}

impl fmt::Display for RedisPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisPoolError::Redis(e) => write!(f, "Redis error: {}", e),
            RedisPoolError::Pool(e) => write!(f, "Connection pool error: {}", e),
        }
    }
}

impl std::error::Error for RedisPoolError {}

impl From<RedisError> for RedisPoolError {
    fn from(err: RedisError) -> Self {
        RedisPoolError::Redis(err)
    }
}

impl From<R2D2Error> for RedisPoolError {
    fn from(err: R2D2Error) -> Self {
        RedisPoolError::Pool(err)
    }
}
