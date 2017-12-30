use std::io::Write;
use std::net::TcpStream;

pub fn send(stream:&mut TcpStream, content: &[u8]) {
    match stream.write(content) {
        Ok(_) => {},
        Err(_e) => {}, // TODO: include error handling logic
    }
}
