

use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;
use std::thread::spawn;
use calc::prelude::*;

// to connect:
//
// ```shell
// telnet 127.0.0.1 8080
// ```

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buf = [0u8; 128];
    println!("Handling client...");

    loop {
        match stream.read(&mut buf) {
            Ok(bytes) if bytes == 0 => {
                continue;
            }
            Ok(bytes) => {
                let string_data = String::from_utf8_lossy(&buf[..bytes]);
                let parsed = match parse(&string_data) {
                    Ok(p) => p,
                    Err(e) => {
                        let formatted = format!("ERROR: {:?}\n", e);
                        stream.write_all(formatted.as_bytes())?;
                        return Ok(());
                    }
                };

                let evaled: i64 = match eval(&parsed) {
                    Ok(ev) => ev,
                    Err(e) => {
                        let formatted = format!("ERROR: {:?}\n", e);
                        stream.write_all(formatted.as_bytes())?;
                        return Ok(());
                    }
                };

                let formatted = format!("= {}\n", evaled);
                stream.write_all(formatted.as_bytes())?;
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
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        // Spawn a thread
        let _ = spawn(move || {
            handle_client(stream?)
        });
    }

    Ok(())
}

