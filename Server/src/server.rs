use crate::http::request;
use std::convert::TryFrom;
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
                            match request::Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!("{}", request);
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
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }
}
