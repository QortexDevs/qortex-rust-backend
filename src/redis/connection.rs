use r2d2_redis::{r2d2, RedisConnectionManager};
use once_cell::sync::OnceCell;
use crate::config::get_settings;

static REDIS_POOL: OnceCell<r2d2::Pool<RedisConnectionManager>> = OnceCell::new();

pub fn get_pool() -> &'static r2d2::Pool<RedisConnectionManager> {
    REDIS_POOL.get_or_init(|| {
            let settings = get_settings();
            let host = settings.redis_host;
            let port = settings.redis_port;
        let manager = RedisConnectionManager::new(format!("redis://{}:{}", host, port))
            .expect("Failed to create Redis connection manager");
        r2d2::Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create Redis pool")
    })
}