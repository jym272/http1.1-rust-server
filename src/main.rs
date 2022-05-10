// #![feature(in_band_lifetimes)]

// #![allow(dead_code, unused_variables)]
mod server;
mod http;
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;
fn main() {

    //load the path in compile time
    let default_public_path = env!("CARGO_MANIFEST_DIR").to_string() + "/public";
    //load the path in runtime, if there is no enviroment variable, use the default path
    //example: cargo run --release  -- --env=PUBLIC_PATH=./public
    let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_public_path);

    let server = Server::new("127.0.0.1:8080");
    server.run(WebsiteHandler::new(public_path)); //run the server forever
}

/* Http request:
GET /user?id=10 HTTP/1.1\r\n
 */



