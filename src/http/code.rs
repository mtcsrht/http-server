/// An HTTP status code with its textual reason phrase.
#[derive(Debug, Clone)]
pub struct HTTPCode {
    pub code: i32,
    pub reason: &'static str,
}

impl HTTPCode {
    pub const fn new(code: i32, reason: &'static str) -> Self {
        Self { code, reason }
    }
}

/// Common status codes.
impl HTTPCode {
    pub const OK: Self = Self::new(200, "OK");
    pub const NOT_FOUND: Self = Self::new(404, "Not Found");
    pub const BAD_REQUEST: Self = Self::new(400, "Bad Request");
    pub const INTERNAL_SERVER_ERROR: Self = Self::new(500, "Internal Server Error");
}
