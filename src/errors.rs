use std::{fmt, io};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// Errors to be expected while parsing
#[derive(Debug)]
pub enum ParsingError {
    /// .bin format error for header "YPBN"
    InvalidMagicHeader,
    /// .bin format error if there's not enough data for record
    WrongRecordData,
    /// Wrong transaction id type, e.g not u64
    WrongTxId,
    /// Wrong transaction type (- TxType)
    WrongTxType,
    /// Wrong transaction status (- TxStatus)
    WrongStatusType,
    /// Empty file
    EmptyFile,
    /// .csv format error if column count in presented file is less than required
    WrongColumnCount(usize, usize, usize),
    /// .csv and .txt format error if key is missing
    MissingKey(String),
    /// .csv and .txt format error if the key is wrong (e.g not presented in Record)
    WrongKey(String),
    /// IoError while reading file
    IoError(io::Error),
    /// Utf8 Error
    Utf8Error(FromUtf8Error),
}

/// Error to be expected while converting
#[derive(Debug)]
pub enum ConvertingError {
    /// IoError while reading/creating file
    IoError(io::Error),
    /// Parsing errors
    Parsing(ParsingError),
    /// Unexpected errors
    Unknown,
}


/// Main logic error in app to be expected
#[derive(Debug)]
pub enum AppError {
    /// IoError while reading/convertings
    Io(io::Error),
    /// Converting errors
    Convert(ConvertingError),
    /// Parsing Errors
    Parse(ParsingError),
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
            ParsingError::Utf8Error(err) => write!(f, "Utf8Error: {}", err)
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<FromUtf8Error> for ParsingError {
    fn from(e: FromUtf8Error) -> Self {
        ParsingError::Utf8Error(e)
    }
}

impl From<ConvertingError> for AppError {
    fn from(e: ConvertingError) -> Self {
        AppError::Convert(e)
    }
}

impl From<ParsingError> for AppError {
    fn from(e: ParsingError) -> Self {
        AppError::Parse(e)
    }
}

impl fmt::Display for ConvertingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertingError::IoError(err) => write!(f, "IO error: {}", err),
            ConvertingError::Unknown => write!(f, "Unknown error occurred"),
            ConvertingError::Parsing(err) => write!(f, "Parsing error: {}", err),
        }
    }
}

impl From<io::Error> for ConvertingError {
    fn from(err: io::Error) -> Self {
        ConvertingError::IoError(err)
    }
}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IoError(err)
    }
}

impl From<ParsingError> for ConvertingError {
    fn from(err: ParsingError) -> Self {
        ConvertingError::Parsing(err)
    }
}

impl std::error::Error for ParsingError {}
impl std::error::Error for ConvertingError {}
