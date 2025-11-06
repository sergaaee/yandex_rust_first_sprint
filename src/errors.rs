use std::{fmt, io};
use std::fmt::write;

#[derive(Debug)]
pub enum ParsingError {
    InvalidMagicHeader,
    WrongRecordData,
    WrongTxId,
    WrongTxType,
    WrongStatusType,
    EmptyFile,
    WrongColumnCount(usize, usize, usize),
    MissingKey(String),
    WrongKey(String),
    IoError(io::Error),
}

#[derive(Debug)]
pub enum ConvertingError {
    IoError(io::Error),
    Unknown,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::InvalidMagicHeader => write!(f, "Invalid magic header"),
            ParsingError::WrongRecordData => write!(f, "Corrupted record data"),
            ParsingError::WrongTxId => write!(f, "Invalid transaction ID"),
            ParsingError::WrongTxType => write!(f, "Invalid transaction type"),
            ParsingError::WrongStatusType => write!(f, "Invalid transaction status"),
            ParsingError::EmptyFile => write!(f, "File is empty"),
            ParsingError::WrongColumnCount(line_num, header, values) => write!(
                f,
                "Error in row {line_num}: expected {header} columns, found {values}"
            ),
            ParsingError::MissingKey(key) => write!(f, "Missing key {key}"),
            ParsingError::WrongKey(key) => write!(f, "Error parsing key {key}"),
            ParsingError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl fmt::Display for ConvertingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertingError::IoError(err) => write!(f, "IO error: {}", err),
            ConvertingError::Unknown => write!(f, "Unknown error occurred")
        }
    }
}

impl From<io::Error> for ConvertingError {
    fn from(err: io::Error) -> Self {
        ConvertingError::IoError(err)
    }
}

impl std::error::Error for ParsingError {}
impl std::error::Error for ConvertingError {}
