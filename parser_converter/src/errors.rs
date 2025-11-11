use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors to be expected while parsing
#[derive(Debug, Error)]
pub enum ParsingError {
    /// .bin format error for header "YPBN"
    #[error("Invalid magic header")]
    InvalidMagicHeader,
    /// .bin format error if there's not enough data for record
    #[error("Corrupted record data")]
    WrongRecordData,
    /// Wrong transaction id type, e.g not u64
    #[error("Invalid transaction id")]
    WrongTxId,
    /// Wrong transaction type (- TxType)
    #[error("Invalid transaction type")]
    WrongTxType,
    /// Wrong transaction status (- TxStatus)
    #[error("Invalid transaction status")]
    WrongStatusType,
    /// Empty file
    #[error("The file is empty")]
    EmptyFile,
    /// .csv format error if column count in presented file is less than required
    #[error("Error in row {line_number:?}: expected {header:?} columns, found {values:?}")]
    WrongColumnCount {
        /// row number
        line_number: usize,
        /// total amount of columns
        header: usize,
        /// received amount of columns
        values: usize,
    },
    /// .csv and .txt format error if key is missing
    #[error("Missing key: {key:?}")]
    MissingKey {
        /// key
        key: String,
    },
    /// .csv and .txt format error if the key is wrong (e.g not presented in Record)
    #[error("Wrong key: {key:?}")]
    WrongKey {
        /// key
        key: String,
    },
    /// IoError while reading file
    #[error("IOError occurred")]
    IoError(#[from] io::Error),
    /// Utf8 Error
    #[error("Utf8Error occurred")]
    Utf8Error(#[from] FromUtf8Error),
}

/// Error to be expected while converting
#[derive(Debug, Error)]
pub enum ConvertingError {
    /// IoError while reading/creating file
    #[error("IOError occurred")]
    IoError(#[from] io::Error),
    /// Parsing errors
    #[error("ParsingError occurred")]
    Parsing(#[from] ParsingError),
    /// Unexpected errors
    #[error("unknown data store error")]
    Unknown,
}

/// Main logic error in app to be expected
#[derive(Debug, Error)]
pub enum AppError {
    /// IoError while reading/converting file
    #[error("IOError occurred")]
    IoError(#[from] io::Error),
    /// Converting errors
    #[error("ConvetingError occurred")]
    Convert(#[from] ConvertingError),
    /// Parsing Errors
    #[error("ParsingError occurred")]
    Parse(#[from] ParsingError),
}
