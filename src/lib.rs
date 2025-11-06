use crate::errors::{ConvertingError, ParsingError};

pub mod bin_format;
pub mod txt_format;
pub mod csv_format;
pub mod errors;

#[derive(Debug, PartialEq)]
pub enum TxType {
    DEPOSIT,
    WITHDRAWAL,
    TRANSFER,
}

#[derive(Debug, PartialEq)]
pub enum TxStatus {
    FAILURE,
    PENDING,
    SUCCESS,
}

#[derive(Debug)]
pub struct Record {
    pub tx_type: TxType,
    pub tx_status: TxStatus,
    pub to_user_id: u64,
    pub from_user_id: u64,
    pub timestamp: u64,
    pub description: String,
    pub tx_id: u64,
    pub amount: u64,
}

pub struct TXTRecord(pub Record);

pub trait Converter {
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, ParsingError>
    where
        Self: Sized;
    fn write_to<W: std::io::Write>(records: &Vec<Record>, writer: &mut W) -> Result<(), ConvertingError>;

    fn as_records(&self) -> &Vec<Record>;
}
