use std::fs;
use std::io::Read;
use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(&path).ok()
                } else {
                    println!("Directory traversal attempt: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/about" => Response::new(StatusCode::Ok, self.read_file("about.html")),
                "/contact" => Response::new(StatusCode::Ok, Some("<h1>Contact</h1>".to_string())),
                // _ => Response::new(StatusCode::NotFound, None),
                path => match self.read_file(path) {
                    Some(file) => Response::new(StatusCode::Ok, Some(file)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}