pub mod bin_format;
pub mod txt_format;
pub mod csv_format;

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
    tx_type: TxType,
    tx_status: TxStatus,
    to_user_id: u64,
    from_user_id: u64,
    timestamp: u64,
    description: String,
    tx_id: u64,
    amount: u64,
}

pub struct TXTRecord(pub Record);

pub trait Converter {
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, String>
    where
        Self: Sized;
    fn write_to<W: std::io::Write>(records: &Vec<Record>, writer: &mut W) -> Result<(), String>;

    fn as_records(&self) -> &Vec<Record>;
}
