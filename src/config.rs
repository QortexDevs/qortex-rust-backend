use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::num::ParseIntError;
use std::sync::RwLock;
use std::env;
use config::ConfigError;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub api_host: String,
    pub api_port: u16,
    pub database_host: String,
    pub database_port: u16,
    pub database_user: String,
    pub database_password: String,
    pub redis_host: String,
    pub redis_port: u16,
}

static SETTINGS: Lazy<RwLock<Settings>> = Lazy::new(|| {
    let settings = load_settings().expect("Failed to load configuration");
    RwLock::new(settings)
});

fn read_str_env(key: &str) -> Result<String, ConfigError> {
    env::var(key)
        .map_err(|e| ConfigError::NotFound(format!("{}: {}", key, e)))
}


fn read_u16_env(key: &str) -> Result<u16, ConfigError> {
    let value = read_str_env(key)?;
    value.parse::<u16>()
        .map_err(|e| ConfigError::Message(format!("Failed to parse {}: {} (must be a number between 0 and 65535)", key, e)))
}

fn load_settings() -> Result<Settings, config::ConfigError> {
    dotenv().ok();


    Ok(Settings { 
        api_host: read_str_env("API_HOST")?,
        api_port: read_u16_env("API_PORT")?,
        database_host: read_str_env("DATABASE_HOST")?,
        database_port: read_u16_env("DATABASE_PORT")?,
        database_user: read_str_env("DATABASE_USER")?,
        database_password: read_str_env("DATABASE_PASSWORD")?,
        redis_host: read_str_env("REDIS_HOST")?,
        redis_port: read_u16_env("REDIS_PORT")?,
    })
}

pub fn get_settings() -> Settings {
    SETTINGS.read().unwrap().clone()
}

// Для изменения настроек в рантайме (если нужно)
// pub fn update_settings(new_settings: Settings) {
//     *SETTINGS.write().unwrap() = new_settings;
// }
