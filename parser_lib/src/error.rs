use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    InvalidFormat(String),
    InvalidField { field: String, reason: String },
    UnexpectedEof,
    InvalidMagic,
    InvalidRecordSize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "IO error: {}", e),
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::InvalidField { field, reason } => {
                write!(f, "Invalid field '{}': {}", field, reason)
            }
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::InvalidMagic => write!(f, "Invalid magic header"),
            ParseError::InvalidRecordSize => write!(f, "Invalid record size"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
