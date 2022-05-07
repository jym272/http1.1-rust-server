mod server;
mod http;

use server::Server;
use http::{Method, Request, MethodError};
//http server using rust
fn main() {

    let server = Server::new("127.0.0.1:8080");
    server.run(); //run the server forever
}

/* Http request:
GET /user?id=10 HTTP/1.1\r\n
 */



