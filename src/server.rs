use std::{
    collections::HashMap,
    io,
    net::{TcpListener, TcpStream},
};

use crate::{
    env::init_env,
    repl::init_repl,
    utils::{remove_port, write_file},
};

pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").expect("Failed to start server");
    println!("Server running on port: {}", init_env().port);

    let mut connections: HashMap<String, Option<TcpStream>> = HashMap::new();

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
                    write_file(crate::handlers::ADDRESSES_PATH, ip);
                }

                connections.insert(ip.to_string(), Some(stream.try_clone().unwrap()));
                handle_client(&mut stream.try_clone().unwrap(), &mut connections).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
fn handle_client(
    stream: &mut TcpStream,
    connections: &mut HashMap<String, Option<TcpStream>>,
) -> Result<(), String> {
    init_repl(stream, connections).unwrap();
    Ok(())
}
