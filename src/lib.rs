#[cfg(test)]
mod unit_tests;

mod util;
mod client;
mod server;

use std::sync::mpsc;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, TcpListener};

pub fn network_init() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
}

pub fn send(stream:&mut TcpStream, content: &[u8]) {
    client::send(stream, content);
}

pub fn recv(listener: TcpListener, count: u8) {
    server::recv(listener, count);
}

pub fn start() -> mpsc::Sender<server::Message> {
    server::start()
}

pub fn stop(sender: mpsc::Sender<server::Message>) {
    server::stop(sender);
}

pub fn deserialize(buf: &[u8]) -> util::Myproto {
    util::deserialize(buf)
}

pub fn serialize(proto: util::Myproto) -> [u8; util::TOTAL_SIZE] {
    util::serialize(proto)
}
