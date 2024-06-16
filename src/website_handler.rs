use crate::{http::{Method, Request, Response, StatusCode}, server::Handler};
use std::fs;

pub struct WebsiteHandler<'p> {
    public_path: &'p str
}

impl<'p> WebsiteHandler<'p> {
    pub fn new(public_path: &'p str) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}",self.public_path,file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(..) => None,
        }
    }
}

impl<'p> Handler for WebsiteHandler<'p> {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, self.read_file("not_found.html")),
                }
            }
            _ => Response::new(StatusCode::NotFound, self.read_file("not_found.html")),
        }
    }
}