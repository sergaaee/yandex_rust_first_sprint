mod common;
use common::write_sample_files;

use parser_converter::{
    Converter, bin_format::BinRecords, csv_format::CSVRecords, txt_format::TXTRecords,
};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

/// Проверяет, что запись и чтение реальных файлов во всех трёх форматах возвращает идентичные данные
#[test]
fn test_all_interconversions_produce_same_data() {
    let dir = PathBuf::from("tests/data");
    write_sample_files(&dir);

    let bin = BinRecords::from_read(&mut File::open(dir.join("sample.bin")).unwrap()).unwrap();
    let csv = CSVRecords::from_read(&mut File::open(dir.join("sample.csv")).unwrap()).unwrap();
    let txt = TXTRecords::from_read(&mut File::open(dir.join("sample.txt")).unwrap()).unwrap();

    assert_records_equal(bin.as_records(), csv.as_records());
    assert_records_equal(csv.as_records(), txt.as_records());
    assert_records_equal(bin.as_records(), txt.as_records());
}

/// Универсальная функция для сравнения двух наборов записей
fn assert_records_equal(a: &[parser_converter::Record], b: &[parser_converter::Record]) {
    assert_eq!(
        a.len(),
        b.len(),
        "Record count mismatch: {} != {}",
        a.len(),
        b.len()
    );
    for (i, (r1, r2)) in a.iter().zip(b.iter()).enumerate() {
        assert_eq!(r1, r2, "Records differ at index {}", i);
    }
}

#[test]
fn test_bin_to_txt_and_back() {
    let dir = PathBuf::from("tests/data");
    write_sample_files(&dir);

    let mut f_bin = BufReader::new(File::open(dir.join("sample.bin")).unwrap());
    let bin_records = BinRecords::from_read(&mut f_bin).unwrap();

    let txt_path = dir.join("tmp_from_bin.txt");
    // Rust не гарантирует запись данных на диск до тех пор пока не вызван flush / writer не дропнут
    {
        let mut txt_writer = BufWriter::new(File::create(&txt_path).unwrap());
        TXTRecords::write_to(bin_records.as_records(), &mut txt_writer).unwrap();
    }

    let mut txt_reader = BufReader::new(File::open(&txt_path).unwrap());
    let txt_records = TXTRecords::from_read(&mut txt_reader).unwrap();

    assert_records_equal(bin_records.as_records(), txt_records.as_records());
}

#[test]
fn test_csv_to_bin_and_back() {
    let dir = PathBuf::from("tests/data");
    write_sample_files(&dir);

    let mut f_csv = BufReader::new(File::open(dir.join("sample.csv")).unwrap());
    let csv_records = CSVRecords::from_read(&mut f_csv).unwrap();

    let bin_path = dir.join("tmp_from_csv.bin");
    // Rust не гарантирует запись данных на диск до тех пор пока не вызван flush / writer не дропнут
    {
        let mut bin_writer = BufWriter::new(File::create(&bin_path).unwrap());
        BinRecords::write_to(csv_records.as_records(), &mut bin_writer).unwrap();
    }

    let mut bin_reader = BufReader::new(File::open(&bin_path).unwrap());
    let bin_records = BinRecords::from_read(&mut bin_reader).unwrap();

    assert_records_equal(csv_records.as_records(), bin_records.as_records());
}

#[test]
fn test_txt_to_csv_and_back() {
    let dir = PathBuf::from("tests/data");
    write_sample_files(&dir);

    let mut f_txt = BufReader::new(File::open(dir.join("sample.txt")).unwrap());
    let txt_records = TXTRecords::from_read(&mut f_txt).unwrap();

    let csv_path = dir.join("tmp_from_txt.csv");
    // Rust не гарантирует запись данных на диск до тех пор пока не вызван flush / writer не дропнут
    {
        let mut csv_writer = BufWriter::new(File::create(&csv_path).unwrap());
        CSVRecords::write_to(txt_records.as_records(), &mut csv_writer).unwrap();
    }

    let mut csv_reader = BufReader::new(File::open(&csv_path).unwrap());
    let csv_records = CSVRecords::from_read(&mut csv_reader).unwrap();

    assert_records_equal(txt_records.as_records(), csv_records.as_records());
}
