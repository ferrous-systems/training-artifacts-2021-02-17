

use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;

// to connect:
//
// ```shell
// telnet 127.0.0.1 8080
// ```

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    // let mut buf = String::new();
    let mut buf = [0u8; 128];
    println!("Handling client...");

    loop {
        match stream.read(&mut buf) {
            Ok(bytes) if bytes == 0 => {
                continue;
            }
            Ok(bytes) => {
                let string_data = String::from_utf8_lossy(&buf[..bytes]);
                println!("{}", string_data);
                stream.write_all(&buf[..bytes])?;
                return Ok(());
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        };
    }


    println!("Done handling!");

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

