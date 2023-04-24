use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handler(mut stream: TcpStream) {
    let mut buf = [0u8; 256];
    stream.read(&mut buf).unwrap();
    println!(
        "\n=== Received ===\n{}\n===   End   ===",
        String::from_utf8_lossy(&buf[..])
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let host = std::env::var("HOST").unwrap_or("localhost".into());
    let port = std::env::var("PORT").unwrap_or("8080".into());

    let endpoint = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&endpoint)?;

    println!("Listening on {}", endpoint);

    for incoming in listener.incoming() {
        let stream = incoming.unwrap();
        handler(stream);
    }

    Ok(())
}
