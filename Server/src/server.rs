use crate::http::{request, response};
use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addres: String,
}

impl Server {
    pub fn new(addres: String) -> Self {
        Self { addres }
    }

    pub fn run(self) {
        println!("Watching {}", self.addres);
        let watcher = TcpListener::bind(&self.addres).unwrap();

        loop {
            match watcher.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request: {}", String::from_utf8_lossy(&buffer));
                            let response = match request::Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!("{}", request);
                                    let path = format!("{}/public/index.html", env!("CARGO_MANIFEST_DIR"));
                                    let body = fs::read_to_string(path).expect("Failed to read file");
                                    response::Response::new(
                                        response::Status::OK,
                                        Some(body),
                                    )
                                }
                                Err(err) => {
                                    println!("{}", err);
                                    response::Response::new(response::Status::BadRequest, None)
                                }
                            };

                            if let Err(err) = response.send_response(&mut stream) {
                                println!("{}", err);
                            }
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }
}
