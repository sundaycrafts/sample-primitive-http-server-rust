use std::error::Error;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handler(stream: TcpStream) {
    let mut buf = Vec::new();

    // read from buffer to reduce system calls
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let res_body = "<h1>Hello, World!</h1>\n";
    let res = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        res_body.len(),
        res_body
    );

    loop {
        let mut chunk = [0u8; 1024];

        // block until it get data from the buffer
        let bytes_read = reader.read(&mut chunk).unwrap();

        if bytes_read == 0 {
            break;
        }

        buf.extend_from_slice(&chunk[..bytes_read]);
    }

    writer.write(res.as_bytes()).unwrap();

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
