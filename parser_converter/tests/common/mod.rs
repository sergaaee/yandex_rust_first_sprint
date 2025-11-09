use parser_converter::{Record, TxStatus, TxType, Converter};
use parser_converter::bin_format::BinRecords;
use parser_converter::csv_format::CSVRecords;
use parser_converter::txt_format::TXTRecords;
use std::fs::File;
use std::path::Path;

pub fn sample_records() -> Vec<Record> {
    vec![
        Record {
            tx_type: TxType::DEPOSIT,
            tx_status: TxStatus::SUCCESS,
            to_user_id: 1,
            from_user_id: 2,
            timestamp: 999_999,
            description: "Sample 1".into(),
            tx_id: 1,
            amount: 500,
        },
        Record {
            tx_type: TxType::TRANSFER,
            tx_status: TxStatus::FAILURE,
            to_user_id: 2,
            from_user_id: 3,
            timestamp: 1_204_598,
            description: "Sample 2".into(),
            tx_id: 2,
            amount: 123_500,
        },
        Record {
            tx_type: TxType::WITHDRAWAL,
            tx_status: TxStatus::SUCCESS,
            to_user_id: 10,
            from_user_id: 2235,
            timestamp: 56_858,
            description: "Sample 3".into(),
            tx_id: 3,
            amount: 546_400,
        },
        Record {
            tx_type: TxType::DEPOSIT,
            tx_status: TxStatus::PENDING,
            to_user_id: 1,
            from_user_id: 2,
            timestamp: 34_564_356,
            description: "Sample 4".into(),
            tx_id: 4,
            amount: 5001,
        },
        Record {
            tx_type: TxType::DEPOSIT,
            tx_status: TxStatus::PENDING,
            to_user_id: 1,
            from_user_id: 2,
            timestamp: 54_670_234,
            description: "Sample 5".into(),
            tx_id: 5,
            amount: 5500,
        },
    ]
}

/// Создаёт реальные файлы .bin .csv .txt в указанной папке `tests/data`
pub fn write_sample_files(dir: &Path) {
    std::fs::create_dir_all(dir).unwrap();
    let records = sample_records();

    // BIN
    let mut f_bin = File::create(dir.join("sample.bin")).unwrap();
    BinRecords::write_to(&records, &mut f_bin).unwrap();

    // CSV
    let mut f_csv = File::create(dir.join("sample.csv")).unwrap();
    CSVRecords::write_to(&records, &mut f_csv).unwrap();

    // TXT
    let mut f_txt = File::create(dir.join("sample.txt")).unwrap();
    TXTRecords::write_to(&records, &mut f_txt).unwrap();
}
