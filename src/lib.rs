#![warn(missing_docs)]
//! YPBank parser and converter library.
//!
//! Provides unified data structures and converters for binary, text, and CSV transaction formats.

use crate::errors::{ConvertingError, ParsingError};
use clap::ValueEnum;

/// Binary format reader/writer implementation
pub mod bin_format;

/// CSV format reader/writer implementation
pub mod csv_format;

/// Common error types for parsing and converting
pub mod errors;

/// TXT format reader/writer implementation
pub mod txt_format;

// Minimal size of fixed part in .bin format w/out description
pub const MIN_FIXED_SIZE: u32 = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4; // = 46 bytes

/// Supported file formats for converting
#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    /// Binary format (`.bin`)
    Bin,
    /// Text format (`.txt`)
    Txt,
    /// Sheet format (`.csv`)
    Csv,
}

impl Format {
    /// Returns the lowercase string representation of the format.
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Bin => "bin",
            Format::Txt => "txt",
            Format::Csv => "csv",
        }
    }
}

/// Transactions types
#[derive(Debug, PartialEq)]
pub enum TxType {
    /// User deposited funds
    DEPOSIT,
    /// User withdrawn funds
    WITHDRAWAL,
    /// Transfer from one user to another
    TRANSFER,
}

/// Transactions statuses
#[derive(Debug, PartialEq)]
pub enum TxStatus {
    /// Failed transaction
    FAILURE,
    /// Pending transaction
    PENDING,
    /// Succesfull transaction
    SUCCESS,
}

/// Structure represents records of transactions
#[derive(Debug)]
pub struct Record {
    /// Transaction type
    pub tx_type: TxType,
    /// Transaction status
    pub tx_status: TxStatus,
    /// User which receives the funds
    pub to_user_id: u64,
    /// Transaction initiator
    pub from_user_id: u64,
    /// Timestamps of the transaction, ms
    pub timestamp: u64,
    /// Transaction description
    pub description: String,
    /// Transaction id
    pub tx_id: u64,
    /// Transaction amount presented in $USD
    pub amount: u64,
}

/// A main interface for reading and writing transaction data in various formats
pub trait Converter {
    /// Reads and parses data from the provided reader
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, ParsingError>
    where
        Self: Sized;

    /// Writes a slice of records to the provided writer
    fn write_to<W: std::io::Write>(
        records: &[Record],
        writer: &mut W,
    ) -> Result<(), ConvertingError>;

    /// Returns internal records
    fn as_records(&self) -> &[Record];
}
