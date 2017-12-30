use network_init;
use send;
use recv;
use start;
use stop;
use util;
    
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
}

#[test]
fn comm_works() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    let mut stream1 = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut stream2 = TcpStream::connect("127.0.0.1:8080").unwrap();
    send(&mut stream1, b"recv test1");
    send(&mut stream2, b"recv test2");
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

    let myproto = util::Myproto {
        w: [1, 1, 1, 1, 1, 1, 1, 1],
        x: [2, 2, 2, 2],
        y: [3, 3],
        z: [4],
    };
    // serialize() function
    let buffer = util::serialize(myproto);
    send(&mut stream1, &buffer[0..util::TOTAL_SIZE]);
    send(&mut stream2, &buffer[0..util::TOTAL_SIZE]);
    // wait for response
    {
        let mut buffer1 = [0; 512];
        stream1.read(&mut buffer1).unwrap();
        // deserialize() function
        let proto1 = util::deserialize(&buffer1[0..util::TOTAL_SIZE]);
        util::display("Client1", proto1);

        stream2.read(&mut buffer1).unwrap();
        // deserialize() function        
        let proto2 = util::deserialize(&buffer1[0..util::TOTAL_SIZE]);
        util::display("Client2", proto2);        
    }

    // serialize() function
    send(&mut stream1, &buffer[0..util::TOTAL_SIZE]);
    send(&mut stream2, &buffer[0..util::TOTAL_SIZE]);
    // wait for response
    {
        let mut buffer1 = [0; 512];
        stream1.read(&mut buffer1).unwrap();
        // deserialize() function
        let proto1 = util::deserialize(&buffer1[0..util::TOTAL_SIZE]);
        util::display("Client1", proto1);

        stream2.read(&mut buffer1).unwrap();
        // deserialize() function        
        let proto2 = util::deserialize(&buffer1[0..util::TOTAL_SIZE]);
        util::display("Client2", proto2);        
    }
    
    send(&mut stream1, &buffer[0..util::TOTAL_SIZE]);
    send(&mut stream2, &buffer[0..util::TOTAL_SIZE]);
    // wait for response
    {
        let mut buffer1 = [0; 512];
        stream1.read(&mut buffer1).unwrap();
        // deserialize() function
        let proto1 = util::deserialize(&buffer1[0..util::TOTAL_SIZE]);
        util::display("Client1", proto1);

        stream2.read(&mut buffer1).unwrap();
        // deserialize() function        
        let proto2 = util::deserialize(&buffer1[0..util::TOTAL_SIZE]);
        util::display("Client2", proto2);        
    }
    
    stop(sender);
    thread::sleep(Duration::from_millis(1000));
    println!("stop server");
}
