use std::collections::HashMap;

/// A parsed incoming HTTP request.
#[derive(Debug, Clone)]
pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HTTPRequest {
    /// Parse a raw HTTP request string into an [`HTTPRequest`].
    pub fn parse(request: &str) -> HTTPRequest {
        let (headers_raw, body) = request.split_once("\r\n\r\n").unwrap_or((request, ""));

        let mut lines = headers_raw.lines();
        let request_line = lines.next().unwrap_or("");

        let mut parts = request_line.split(' ');
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");

        let mut headers = HashMap::new();

        lines.for_each(|line| {
            let (key, value) = line.split_once(": ").unwrap_or(("", ""));
            headers.insert(key.to_string(), value.to_string());
        });

        HTTPRequest {
            method: method.to_string(),
            path: path.to_string(),
            headers,
            body: Some(body.to_string()),
        }
    }

    /// Print the request to stdout (useful for debugging).
    pub fn print(&self) {
        println!("Method: {}\nPath: {}\n", self.method, self.path);

        self.headers.iter().for_each(|(key, value)| {
            println!("{key}: {value}");
        });

        println!("{}", self.body.clone().unwrap_or_default());
    }
}
