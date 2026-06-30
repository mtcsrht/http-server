mod http;

use std::{io::{BufReader, Read}, net::TcpListener};

use http::HTTPRequest;

fn main() {
    let server = HTTPServer::new("127.0.0.1:8080");
    server.run();
}

struct HTTPServer {
    addr: String,
}

impl HTTPServer {
    pub fn new(addr: &str) -> HTTPServer {
        let addr = addr.to_string();
        HTTPServer { addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((socket, _addr)) => {
                    Self::handle_client(socket);
                }
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }
    }

    fn handle_client(socket: std::net::TcpStream) {
        let mut reader = BufReader::new(&socket);
        let mut buffer = [0; 1024];

        // TODO handle requests by their method
        match reader.read(&mut buffer) {
            Ok(bytes_read) => {
                let received = String::from_utf8_lossy(&buffer[..bytes_read]);
                let req = HTTPRequest::parse(&received);
                req.print();
            }
            Err(e) => eprintln!("Failed to read from socket: {e}"),
        }
    }
}
