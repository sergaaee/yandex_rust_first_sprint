use clap::Parser;
use parser_converter::Converter;
use parser_converter::errors::{AppError, ParsingError};
use parser_converter::{bin_format::BinRecords, csv_format::CSVRecords, txt_format::TXTRecords};
use std::{fs::File, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "ypbank_compareer")]
#[command(about = "compares YPBank transaction records between formats")]
struct Args {
    /// Input first file path
    #[arg(long)]
    file1: PathBuf,

    /// Input second file path
    #[arg(long)]
    file2: PathBuf,
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();
    let first_file_format = detect_format(&args.file1).unwrap_or_else(|| "unknown".to_string());
    let second_file_format = detect_format(&args.file2).unwrap_or_else(|| "unknown".to_string());

    let mut file1 = File::open(&args.file1)?;
    let mut file2 = File::open(&args.file2)?;

    // читаем входной формат и записываем в новый
    match (first_file_format.as_str(), second_file_format.as_str()) {
        ("txt", "txt") => compare::<TXTRecords, TXTRecords>(&mut file1, &mut file2)?,
        ("bin", "txt") => compare::<BinRecords, TXTRecords>(&mut file1, &mut file2)?,
        ("bin", "csv") => compare::<BinRecords, CSVRecords>(&mut file1, &mut file2)?,

        ("bin", "bin") => compare::<BinRecords, BinRecords>(&mut file1, &mut file2)?,
        ("txt", "bin") => compare::<TXTRecords, BinRecords>(&mut file1, &mut file2)?,
        ("txt", "csv") => compare::<TXTRecords, CSVRecords>(&mut file1, &mut file2)?,

        ("csv", "csv") => compare::<CSVRecords, CSVRecords>(&mut file1, &mut file2)?,
        ("csv", "bin") => compare::<CSVRecords, BinRecords>(&mut file1, &mut file2)?,
        ("csv", "txt") => compare::<CSVRecords, TXTRecords>(&mut file1, &mut file2)?,

        _ => {
            eprintln!(
                "❌ Unknown format combination: {} → {}",
                first_file_format, second_file_format
            );
            std::process::exit(1);
        }
    }

    println!("Files are identical");

    Ok(())
}

fn compare<First, Second>(first_file: &mut File, second_file: &mut File) -> Result<(), ParsingError>
where
    First: Converter,
    Second: Converter,
{
    let mut current_record = 0;
    let first_records = First::from_read(first_file)?;
    let second_records = Second::from_read(second_file)?;

    let f_len = &first_records.as_records().len();
    let s_len = &second_records.as_records().len();

    if f_len != s_len {
        println!(
            "The count of transactions isn't equal: {} vs {}",
            f_len, s_len
        )
    }

    for (first_record, second_record) in first_records
        .as_records()
        .iter()
        .zip(second_records.as_records().iter())
    {
        current_record += 1;
        if first_record.tx_type != second_record.tx_type {
            println!(
                "Not equal: tx_type differs: {:?} vs {:?} for record number {current_record}",
                first_record.tx_type, second_record.tx_type
            );
        }
        if first_record.tx_status != second_record.tx_status {
            println!(
                "Not equal: tx_status differs: {:?} vs {:?} for record number {current_record}",
                first_record.tx_status, second_record.tx_status
            );
        }
        if first_record.to_user_id != second_record.to_user_id {
            println!(
                "Not equal: to_user_id differs: {} vs {} for record number {current_record}",
                first_record.to_user_id, second_record.to_user_id
            );
        }
        if first_record.from_user_id != second_record.from_user_id {
            println!(
                "Not equal: from_user_id differs: {} vs {} for record number {current_record}",
                first_record.from_user_id, second_record.from_user_id
            );
        }
        if first_record.timestamp != second_record.timestamp {
            println!(
                "Not equal: timestamp differs: {} vs {} for record number {current_record}",
                first_record.timestamp, second_record.timestamp
            );
        }
        if first_record.description != second_record.description {
            println!(
                "Not equal: description differs: {:?} vs {:?} for record number {current_record}",
                first_record.description, second_record.description
            );
        }
        if first_record.tx_id != second_record.tx_id {
            println!(
                "Not equal: tx_id differs: {} vs {} for record number {current_record}",
                first_record.tx_id, second_record.tx_id
            );
        }
        if first_record.amount != second_record.amount {
            println!(
                "Not equal: amount differs: {} vs {} for record number {current_record}",
                first_record.amount, second_record.amount
            );
        }
    }

    Ok(())
}

fn detect_format(path: &PathBuf) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}
