fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addres: String,
}

impl Server {
    fn new(addres: String) -> Self {
        Self { addres }
    }

    fn run(self) {
        println!("Watching {}", self.addres)
    }
}

struct Request {
    function: Function,
    path: String,
    query: Option<String>,
}

enum Function {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
