use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:42069").expect("Failed to start server");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap().to_string();
                println!("Connected to :{}", addr);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
