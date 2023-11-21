mod env;

use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    time::Duration,
};

use env::init_env;

fn connect(attempts: i32) {
    match TcpStream::connect(init_env().server_url) {
        Ok(stream) => {
            let reader = BufReader::new(&stream);
            for line in reader.lines().flatten() {}
            connect(init_env().retry_attempts);
        }
        Err(_) => {
            retry_connection(attempts);
        }
    }
}

fn main() {
    connect(init_env().retry_attempts);
}

fn retry_connection(attempts: i32) {
    if attempts == 0 {
        std::process::exit(-1);
    }
    std::thread::sleep(Duration::from_secs(init_env().retry_interval as u64 * 60));
    connect(attempts - 1);
}