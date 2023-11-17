use std::io;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Sup bitch");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    match listener.accept().await {
        Ok((_socket, addr)) => println!("connected to {:?}", addr),
        Err(e) => eprintln!("error {:?}", e),
    }

    Ok(())
}
