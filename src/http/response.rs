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
use std::io::{Write};

pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response {
            status_code,
            body,
        }
    }
    // dynamic dispatch to the correct writer -> dyn: uses v-table and have a runtime cost.
    // static dispatch to the correct writer -> imp: uses static dispatch and have a compile time cost. Larger binary size.
    // the compiler with static dispatch finds all the usages of "send" and know all types that can stream have.
    // then the compiler generates different code 'send fn' for each type.

   pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
       let body: &str = match &self.body {
           Some(body) => body,
           None => "",
       };

       write!(stream, "HTTP/1.1 {} OK\r\n", self.status_code)?;
       //can be also text/css
       // write!(stream, "Content-Type: text/html; charset=utf-8\r\n")?;
       write!(stream, "Content-Length: {}\r\n", body.len())?;
       write!(stream, "Date: Sun, 27 Nov 2017 00:00:00 GMT\r\n")?;

       write!(stream, "Server: Rust/1.0.0 (linux) (rustc 1.0.0) (20171127) (x86_64-unknown-linux-gnu)\r\n")?;
       write!(stream, "Connection: close\r\n")?;
       write!(stream, "Last-Modified: Sun, 27 Nov 2017 00:00:00 GMT\r\n")?;
       write!(stream, "ETag: \"1234\"\r\n")?;
       write!(stream, "Accept-Ranges: bytes\r\n")?;
       write!(stream, "Cache-Control: max-age=0, no-cache, no-store, must-revalidate\r\n")?;
       write!(stream, "Expires: Sun, 27 Nov 2017 00:00:00 GMT\r\n")?;
       write!(stream, "Pragma: no-cache\r\n")?;
       write!(stream, "\r\n")?;
       write!(stream, "{}", body)

   }
}

impl Display for Response{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let body: &str = match &self.body {
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
























