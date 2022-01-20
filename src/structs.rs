use std::io::Error;

pub struct Header {
    pub valid: bool,
}

pub enum ErrorKind {
    INVALID_JPEG,
    EXPECTED_A_MARKER,
    UNEXPECTED_ERROR
}

impl From<std::io::Error> for ErrorKind {
    fn from(e: Error) -> Self {
        ErrorKind::UNEXPECTED_ERROR
    }
}

impl Default for Header {
    fn default() -> Self {
        Header { valid: true }
    }
}