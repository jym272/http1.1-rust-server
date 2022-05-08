// #![feature(in_band_lifetimes)]

// #![allow(dead_code, unused_variables)]
mod server;
mod http;

use server::Server;
fn main() {

    let server = Server::new("127.0.0.1:8080");
    server.run(); //run the server forever
}

/* Http request:
GET /user?id=10 HTTP/1.1\r\n
 */



