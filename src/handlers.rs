use std::{
    collections::HashMap,
    io::{self, Write},
    net::TcpStream,
};

use crate::utils::read_file;

// Handlers for network connections
//
// Constants
// const REMOTE_EXEC_PAYLOAD_TYPE: &str = "execute";
pub const ADDRESSES_PATH: &str = "./addresses.txt";

fn get_addresses() -> io::Result<Vec<String>> {
    Ok(read_file(ADDRESSES_PATH)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect())
}

fn write(conn: &mut TcpStream, payload: &str) -> io::Result<()> {
    conn.write_all(format!("{}\n", payload.to_owned()).as_bytes())?;
    Ok(())
}
fn ping_connection(conn: &mut TcpStream) -> io::Result<()> {
    write(conn, "ping")?;
    Ok(())
}

fn online_connection_count(connections: &mut HashMap<String, Option<TcpStream>>) -> usize {
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

pub fn get_connections(connections: &mut HashMap<String, Option<TcpStream>>) -> io::Result<()> {
    for addr in get_addresses()? {
        if addr.is_empty() {
            continue;
        }
        connections.insert(addr, None);
    }
    let count = online_connection_count(connections);
    println!("Number of  connections: {}", count,);
    Ok(())
}
