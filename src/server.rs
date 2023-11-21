use std::{
    collections::HashMap,
    io::{self, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    env::init_env,
    handlers::get_connections,
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

                get_connections(&mut connections)?;
                connections.insert(ip.to_string(), Some(stream.try_clone().unwrap()));
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
fn handle_client(mut stream: TcpStream) -> Result<(), String> {
    loop {
        print!("Input: ");
        io::stdout().flush().expect("failed to get it");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|err| err.to_string())?;
        stream.write_all(input.as_bytes()).unwrap();
    }
}
