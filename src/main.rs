use args::{
    Arguments,
    Commands::{Connections, Execute},
};
use clap::Parser;
use std::collections::HashMap;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    net::{TcpListener, TcpStream},
    process,
};

mod args;

fn main() {
    let args = Arguments::parse();
    match args.sub_commands {
        Connections => println!("sup bitch"),
        Execute(val) => println!("whoa! :{:?}", val.commands),
    }
    // let listener = TcpListener::bind("127.0.0.1:42069").expect("Failed to start server");
    // println!("Server running on port :{}", init_env().port);

    // let mut connections: HashMap<String, TcpStream> = HashMap::new();

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             let addr = stream.peer_addr().unwrap().to_string();
    //             println!("Connected to :{}", addr);
    //             let ip = remove_port(&addr);

    //             if connections.len() == init_env().connection_limit as usize {
    //                 drop(stream);
    //                 println!("Maximum connection reached!");
    //                 continue;
    //             }

    //             if !connections.contains_key(ip) {
    //                 write_file(ADDRESSES_PATH, ip);
    //             }

    //             connections.insert(ip.to_string(), stream.try_clone().unwrap());
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to accept connection: {}", e);
    //         }
    //     }
    // }
}
const ADDRESSES_PATH: &str = "./addresses.txt";
// Utility functions for
// file system management
//
// Creates the file
fn create_file(path: &str) -> bool {
    match File::create(path) {
        Ok(_) => true,
        Err(error) => {
            eprintln!("Error creating file :{}", error);
            false
        }
    }
}

// Writes to file
fn write_file(path: &str, data: &str) {
    match OpenOptions::new().append(true).write(true).open(path) {
        Ok(mut file) => {
            file.write_all(format!("{}\n", data).as_bytes())
                .expect("Error writing to file");
        }
        Err(error) => eprintln!("Error opening file :{}", error),
    }
}

// Parses x.x.x.x:pppp by spliting at ':'
// and returns the ip x.x.x.x
fn remove_port(addr: &str) -> &str {
    addr.split(':').next().unwrap()
}

// Handlers for network connections
//
// Struct
#[derive(Debug)]
struct ServerEnvConfig {
    port: String,
    connection_limit: i32,
    keep_alive_interval: i32,
}

impl ServerEnvConfig {
    fn new() -> Result<Self, String> {
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

fn init_env() -> ServerEnvConfig {
    match ServerEnvConfig::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load env config: {}", err);
            process::exit(1);
        }
    }
}
