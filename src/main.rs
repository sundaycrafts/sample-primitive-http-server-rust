use std::error::Error;
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn Error>> {
    let host = std::env::var("HOST").unwrap_or("localhost".into());
    let port = std::env::var("PORT").unwrap_or("8080".into());

    let endpoint = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&endpoint)?;

    println!("Listening on {}", endpoint);

    for incoming in listener.incoming() {
        let stream = incoming.unwrap();
        println!("New connection from: {}", stream.peer_addr()?);
    }

    Ok(())
}
