use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub enum Status {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl Status {
    fn mesaage(&self) -> &str {
        match self {
            Status::OK => "200 OK",
            Status::BadRequest => "400 BadRequest",
            Status::NotFound => "404 NotFonud",
        }
    }
}

#[derive(Debug)]
pub struct Response {
    status: Status,
    body: Option<String>,
}

impl Response {
    pub fn new(status: Status, body: Option<String>) -> Self {
        Self { status, body }
    }

    pub fn send_response(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(text) => text,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} \r\n\r\n{}",
            self.status.mesaage(),
            body
        )
    }
}
