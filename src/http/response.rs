use std::collections::HashMap;

use super::{code::HTTPCode, status_line::HTTPStatusline};

/// An outgoing HTTP response.
#[derive(Debug, Clone)]
pub struct HTTPResponse {
    pub status_line: HTTPStatusline,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HTTPResponse {
    /// Create a new response with the given status code.
    pub fn new(code: HTTPCode) -> Self {
        Self {
            status_line: HTTPStatusline::new("HTTP/1.1", code),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Set the response body.
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Add a header to the response.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Serialize the response into a raw HTTP string.
    pub fn to_string(&self) -> String {
        let mut out = self.status_line.as_str();
        out.push_str("\r\n");

        for (key, value) in &self.headers {
            out.push_str(&format!("{key}: {value}\r\n"));
        }

        out.push_str("\r\n");

        if let Some(ref body) = self.body {
            out.push_str(body);
        }

        out
    }
}
