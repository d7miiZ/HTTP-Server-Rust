pub struct Server {
    addres: String,
}

impl Server {
    pub fn new(addres: String) -> Self {
        Self { addres }
    }

    pub fn run(self) {
        println!("Watching {}", self.addres)
    }
}
