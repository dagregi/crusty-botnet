use std::{env, process};

// Struct
#[derive(Debug)]
pub struct ServerEnvConfig {
    pub port: String,
    pub connection_limit: i32,
    pub keep_alive_interval: i32,
}

impl ServerEnvConfig {
    pub fn new() -> Result<Self, String> {
        dotenv::dotenv().ok();
        let port = env::var("PORT").map_err(|_| "PORT not found")?;
        let connection_limit =
            env::var("CONNECTION_LIMIT").map_err(|_| "CONNECTION_LIMIT not found")?;
        let keep_alive_interval =
            env::var("KEEP_ALIVE_INTERVAL").map_err(|_| "KEEP_ALIVE_INTERVAL not found")?;

        Ok(Self {
            port,
            connection_limit: connection_limit
                .parse()
                .map_err(|_| "CONNECTION_LIMIT must be a number")?,
            keep_alive_interval: keep_alive_interval
                .parse()
                .map_err(|_| "KEEP_ALIVE_INTERVAL must be a number")?,
        })
    }
}

pub fn init_env() -> ServerEnvConfig {
    match ServerEnvConfig::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load env config: {}", err);
            process::exit(1);
        }
    }
}
