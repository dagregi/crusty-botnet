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

// TODO: too many unwraps fix it
pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").expect("Failed to start server");
    println!("Server running on port: {}", init_env().port);

    let mut connections: HashMap<String, Option<TcpStream>> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap().to_string();
                let ip = remove_port(&addr);

                if connections.len() == init_env().connection_limit as usize {
                    drop(stream);
                    println!("Maximum connection reached!");
                    continue;
                }

                if !connections.contains_key(ip) {
                    write_file(crate::handlers::ADDRESSES_PATH, ip);
                }

                connections.insert(ip.to_string(), Some(stream.try_clone().unwrap()));
                // TODO: need to use a thread pool for multiple clients
                init_repl(&mut stream.try_clone().unwrap(), &mut connections).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
