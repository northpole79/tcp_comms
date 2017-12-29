use std::io::Write;
use std::net::TcpStream;
use std::str;

pub fn send(stream:&mut TcpStream, content: &str) {
    match stream.write_fmt(format_args!("{}", content)) {
        Ok(_) => {},
        Err(_e) => {}, // TODO: include error handling logic
    }
}
