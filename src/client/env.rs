use std::{env, process};

pub struct ClientEnvConfig {
    pub server_url: String,
    pub retry_interval: i32,
    pub retry_attempts: i32,
}

impl ClientEnvConfig {
    pub fn new() -> Result<Self, String> {
        dotenv::dotenv().ok();
        let server_url = env::var("SERVER_URL").map_err(|_| "SERVER_URL not found")?;
        let retry_interval = env::var("RETRY_INTERVAL").map_err(|_| "RETRY_INTERVAL not found")?;
        let retry_attempts = env::var("RETRY_ATTEMPTS").map_err(|_| "RETRY_ATTEMPTS not found")?;

        Ok(Self {
            server_url,
            retry_interval: retry_interval
                .parse()
                .map_err(|_| "RETRY_INTERVAL must be a number")?,
            retry_attempts: retry_attempts
                .parse()
                .map_err(|_| "RETRY_ATTEMPTS must be a number")?,
        })
    }
}

pub fn init_env() -> ClientEnvConfig {
    match ClientEnvConfig::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load env config: {}", err);
            process::exit(1);
        }
    }
}
