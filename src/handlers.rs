use std::{
    collections::HashMap,
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    env::init_env,
    utils::{read_file, remove_port, write_file},
};

// Handlers for network connections
//
// Constants
// const REMOTE_EXEC_PAYLOAD_TYPE: &str = "execute";
const ADDRESSES_PATH: &str = "./addresses.txt";

fn get_addresses() -> io::Result<Vec<String>> {
    Ok(read_file(ADDRESSES_PATH)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect())
}

pub fn write(conn: &mut TcpStream, payload: &str) -> io::Result<()> {
    conn.write_all((payload.to_owned() + "\n").as_bytes())?;
    Ok(())
}

pub fn ping_connection(conn: &mut TcpStream) -> io::Result<()> {
    write(conn, "ping")?;
    Ok(())
}

pub fn online_connection_count(connections: &mut HashMap<String, Option<TcpStream>>) -> usize {
    let mut count = 0;
    let mut remove_keys = Vec::new();
    for (addr, conn) in connections.iter_mut() {
        if let Some(conn) = conn {
            if ping_connection(conn).is_err() {
                remove_keys.push(addr.clone());
                continue;
            }
            count += 1;
        }
    }
    for key in remove_keys {
        connections.remove(&key);
    }
    count
}

pub fn init(connections: &mut HashMap<String, Option<TcpStream>>) -> io::Result<()> {
    for addr in get_addresses()? {
        if addr.is_empty() {
            continue;
        }
        connections.insert(addr, None);
    }
    let count = online_connection_count(connections);
    println!("Number of  connections: {}", count);
    Ok(())
}

pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").expect("Failed to start server");
    println!("Server running on port: {}", init_env().port);

    let mut connections: HashMap<String, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap().to_string();
                let ip = remove_port(&addr);
                println!("Connected to: {}", ip);

                if connections.len() == init_env().connection_limit as usize {
                    drop(stream);
                    println!("Maximum connection reached!");
                    continue;
                }

                if !connections.contains_key(ip) {
                    write_file(ADDRESSES_PATH, ip);
                }

                connections.insert(ip.to_string(), stream.try_clone().unwrap());
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
