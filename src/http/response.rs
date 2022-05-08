//Server response:
// HTTP/1.1 200 OK
// Content-Type: text/html; charset=utf-8
// Content-Length: 12
// Date: Sun, 27 Nov 2017 00:00:00 GMT
// Server: Rust/1.0.0 (linux) (rustc 1.0.0) (20171127) (x86_64-unknown-linux-gnu)
// Connection: close
// Last-Modified: Sun, 27 Nov 2017 00:00:00 GMT
// ETag: "1234"
// Accept-Ranges: bytes
// Cache-Control: max-age=0, no-cache, no-store, must-revalidate
// Expires: Sun, 27 Nov 2017 00:00:00 GMT
// Pragma: no-cache
//
// Hello, world!

use std::fmt::Display;
use std::net::TcpStream;
use crate::http::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response {
            status_code,
            body,
        }
    }
    pub fn send(&self, stream: &mut TcpStream)-> Result<(), std::io::Error> {
        let status_line = format!("HTTP/1.1 {} OK\r\n", self.status_code);
        stream.write(status_line.as_bytes())?;
    }
}

impl Display for Response{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let body = match &self.body {
            Some(body) => body,
            None => "",
        };


        write!(f, "HTTP/1.1 {} OK\r\n", self.status_code)?;
        write!(f, "Content-Type: text/html; charset=utf-8\r\n")?;
        write!(f, "Content-Length: {}\r\n", body.len())?;
        write!(f, "Date: Sun, 27 Nov 2017 00:00:00 GMT\r\n")?;

        write!(f, "Server: Rust/1.0.0 (linux) (rustc 1.0.0) (20171127) (x86_64-unknown-linux-gnu)\r\n")?;
        write!(f, "Connection: close\r\n")?;
        write!(f, "Last-Modified: Sun, 27 Nov 2017 00:00:00 GMT\r\n")?;
        write!(f, "ETag: \"1234\"\r\n")?;
        write!(f, "Accept-Ranges: bytes\r\n")?;
        write!(f, "Cache-Control: max-age=0, no-cache, no-store, must-revalidate\r\n")?;
        write!(f, "Expires: Sun, 27 Nov 2017 00:00:00 GMT\r\n")?;
        write!(f, "Pragma: no-cache\r\n")?;
        write!(f, "\r\n")?;
        write!(f, "{}", body)
    }

}
























