use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&self, req: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParseError) -> Response{
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest,Some("<h1>Bad Request</h1>".to_string()))
    }
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Server {
            addr: addr.to_string(),
        }
    }
    pub fn run(&self, handler: impl Handler) {
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
                                let buf_ = &buf[..n];
                                println!("{}", String::from_utf8_lossy(buf_));

                                let response = match Request::try_from(buf_){
                                    Ok(request) => {

                                        handler.handle_request(&request)
                                    }
                                    Err(e) => {

                                        handler.handle_bad_request(&e)
                                    }
                                };
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send a response: {}", e);
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