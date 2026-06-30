use super::code::HTTPCode;

/// The first line of an HTTP message (e.g. `HTTP/1.1 200 OK`).
#[derive(Debug, Clone)]
pub struct HTTPStatusline {
    pub version: String,
    pub code: HTTPCode,
}

impl HTTPStatusline {
    pub fn new(version: impl Into<String>, code: HTTPCode) -> Self {
        Self {
            version: version.into(),
            code,
        }
    }

    /// Format as a raw status-line string: `HTTP/1.1 200 OK`.
    pub fn as_str(&self) -> String {
        format!("{}/{} {}", self.version, self.code.code, self.code.reason)
    }
}
