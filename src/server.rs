use std::io::Read;
use crate::{Request};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}


impl Server {
    pub fn new(addr: &str) -> Self {
        Server {
            addr: addr.to_string(),
        }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                        let mut buf = [0; 1024]; // buffer to store data, 0 in all bytes
                        match stream.read(&mut buf) {
                            Ok(n) if n == 0 => {
                                println!("Client disconnected");
                            },
                            Ok(n) => {
                                let buf = &buf[..n];
                                println!("{}", String::from_utf8_lossy(buf));
                                match Request::try_from(buf){
                                    Ok(request) => {
                                        // println!("{:?}", request);
                                    }
                                    Err(e) => {
                                        println!("Failed to parse a request: {}", e);
                                    }

                                }
                            },
                            Err(e) => {
                                println!("Failed to read from connection: {}", e);
                            }
                    }

                    // println!("New connection!");
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                    // continue 'listener;
                }
                // _ => {
                //     println!("Unknown error!");
                // }

            }
        }


        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();
        //
        // }
    }
}