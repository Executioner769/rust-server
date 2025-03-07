use std::{convert::TryFrom, io::Read, net::TcpListener};
use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BasRequest, None)
    }
}
pub struct Server {
        addr: String,
    }
    
    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }
    
        pub fn run(self, mut handler: impl Handler) {
            let listener = TcpListener::bind(&self.addr).unwrap();

            println!("Listening on {}", self.addr);
            
            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(..) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buf));
                                // let res: &Result<Request, _> = &buf[..].try_into();
                                let response = match Request::try_from(&buf[..]) {
                                    Ok(request) => {
                                        handler.handle_request(&request)
                                    }
                                    Err(e) => {
                                        handler.handle_bad_request(&e)
                                    },
                                };

                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response {}", e)
                                }
                            }
                            Err(e) => println!("Failed to read form connection: {}", e),
                        }
                    }
                    Err(e) => println!("Failed to establish a connection: {}", e),
                }
            }
        }
    }