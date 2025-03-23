use std::io::{Write, Error};
use std::net::TcpStream;

pub fn send_message_tcp(message: &str) -> Result<(), Error> {
    let mut stream = TcpStream::connect("localhost:9080")?;
    stream.write_all(message.as_bytes())?;
    stream.flush()?;
    println!("Mensaje enviado con éxito a localhost:9080");
    Ok(())
}