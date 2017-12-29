use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};

pub enum Message {
    Terminate,
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    println!("Conn Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    match stream.write_fmt(format_args!("{}",String::from_utf8_lossy(&buffer[..]))) {
        Ok(_) => {},
        Err(_e) => {}, // TODO: include error handling logic                                                
    }

    stream.flush().unwrap();    
}

pub fn start() -> mpsc::Sender<Message> {
    let (accept_sender, accept_receiver) = mpsc::channel();    
    let accept_receiver = Arc::new(Mutex::new(accept_receiver)); // convert to heap/global controlled variable
    thread::spawn(move || {
        handle_accept(Arc::clone(&accept_receiver));
    });
    accept_sender
}

fn handle_accept(accept_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    
    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                thread::spawn(move || {
                    handle_stream(stream);
                });
            },
            Err(_e) => {}, //TODO: include error handling logic
        }
        match accept_receiver.lock().unwrap().try_recv() {
            Ok(Message::Terminate) |
            Err(TryRecvError::Disconnected) => {
                println!("Accept terminating");
                // recv threads terminate after receiving an EOF
                break;
            },
            Err(TryRecvError::Empty) => {},
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(0) => {
                // EOF reached implying termination of connection
                println!("Worker terminating");
                break;
            },
            Ok(_) => {
                println!("Msg Request: {}", String::from_utf8_lossy(&buffer[..]));
                
                match stream.write_fmt(format_args!("{}",String::from_utf8_lossy(&buffer[..]))) {
                    Ok(_) => {},
                    Err(_e) => {}, // TODO: include error handling logic
                }

                stream.flush().unwrap();
            },
            Err(_e) => {}, // TODO include error handling logic
        }
    }
}

pub fn stop(sender: mpsc::Sender<Message>) {
    println!("Terminating Server");
    sender.send(Message::Terminate).unwrap();
}

pub fn recv(listener: TcpListener, count: u8) {
    let mut iter = 0;
    loop {
        match listener.accept() {
            Ok((stream, _addr)) => handle_connection(stream),
            Err(_e) => {}, //TODO include error handling logic
        }
        iter += 1;
        if iter == count { break; }
    }
}
