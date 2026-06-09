use std::{collections::HashMap, io::{BufReader, Read}, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        match listener.accept() {
            Ok((socket, _addr)) => {
                handle_client(socket);
            }
                ,
            Err(e) => println!("couldn't get client: {e:?}"),
        }
        
    }
}
struct HTTPRequest{
    method: String, 
    path: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HTTPRequest{
    pub fn parse(request: &str) -> HTTPRequest{
       let (headers_raw, body) = request.split_once("\r\n\r\n").unwrap_or((request, ""));
       
       let mut lines = headers_raw.lines();
       let reqest_line = lines.next().unwrap_or("");

       let mut parts = reqest_line.split(' ');
       let method = parts.next().unwrap_or("");
       let path = parts.next().unwrap_or("");

       let mut headers = HashMap::new();

       lines.for_each(|line| {
            let (key , value) = line.split_once(": ").unwrap_or(("", ""));
            headers.insert(key.to_string(), value.to_string());
       });

       return HTTPRequest { method: method.to_string(), path: path.to_string(), headers: headers, body: Option::Some(body.to_string()) }
    }
    pub fn print(&self){
        println!("Method: {}\nPath: {}\n", self.method, self.path);

        self.headers.iter().for_each(|line| {
            println!("{}: {}",line.0.to_string(), line.1.to_string())
        });
        
        println!("{}",self.body.clone().unwrap_or("".to_string()));
    }
}

fn handle_client(socket: std::net::TcpStream) {
    let mut reader = BufReader::new(&socket);
    let mut buffer = [0; 1024];

    match reader.read(&mut buffer) {
        Ok(bytes_read) => {
            let received = String::from_utf8_lossy(&buffer[..bytes_read]);
            let req = HTTPRequest::parse(&received);
            req.print();
        },
        Err(e) => eprintln!("Failed to read from socket: {e}") 
    }

}
