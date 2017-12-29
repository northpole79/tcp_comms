#[cfg(test)]
mod tests {
    use network_init;
    use send;
    use recv;
    use start;
    use stop;
    
    use std::io::prelude::*;
    use std::thread;
    use std::time::Duration;
    use std::net::{TcpListener, TcpStream};
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn network_works() {
        let socket = network_init();
        assert_eq!(socket.is_ipv4(), true);
//        assert_eq!(socket.is_ipv6(), true);
    }

    #[test]
    fn comm_works() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        let mut stream1 = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut stream2 = TcpStream::connect("127.0.0.1:8080").unwrap();  
        send(&mut stream1, "recv test1");
        send(&mut stream2, "recv test2");
        recv(listener, 10);
    }

    #[test]
    fn server_works() {
        let sender = start();
        thread::sleep(Duration::from_millis(500));
        
        let mut stream1 = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut stream2 = TcpStream::connect("127.0.0.1:8080").unwrap();
        stream1.set_nonblocking(false).expect("blocking call failed");
        stream2.set_nonblocking(false).expect("blocking call failed");
        
        send(&mut stream1, "threaded test1");
        send(&mut stream2, "threaded test2");
        // wait for response
        {       
            let mut buffer1 = [0; 512];
            stream1.read(&mut buffer1).unwrap();
            println!("buffer1: {}", String::from_utf8_lossy(&buffer1[..]));
            
            let mut buffer2 = [0; 512];
            stream2.read(&mut buffer2).unwrap();
            println!("buffer2: {}", String::from_utf8_lossy(&buffer2[..]));
        }

        send(&mut stream1, "threaded test11");        
        send(&mut stream2, "threaded test22");
        // wait for response
        {       
            let mut buffer1 = [0; 512];
            stream1.read(&mut buffer1).unwrap();
            println!("buffer1: {}", String::from_utf8_lossy(&buffer1[..]));
            
            let mut buffer2 = [0; 512];
            stream2.read(&mut buffer2).unwrap();
            println!("buffer2: {}", String::from_utf8_lossy(&buffer2[..]));
        }
        send(&mut stream1, "threaded test111");        
        send(&mut stream2, "threaded test222");

        {       
            let mut buffer1 = [0; 512];
            stream1.read(&mut buffer1).unwrap();
            println!("buffer1: {}", String::from_utf8_lossy(&buffer1[..]));
            
            let mut buffer2 = [0; 512];
            stream2.read(&mut buffer2).unwrap();
            println!("buffer2: {}", String::from_utf8_lossy(&buffer2[..]));
        }
        
        stop(sender);
        thread::sleep(Duration::from_millis(1000));        
        println!("stop server");
    }
}

mod client;
mod server;

use std::sync::mpsc;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, TcpListener};
use std::str;

pub fn network_init() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
}

pub fn send(stream:&mut TcpStream, content: &str) {
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
