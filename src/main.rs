use parser_converter::Converter;
use parser_converter::bin_format::BinRecords;
use parser_converter::txt_format::TXTRecords;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use parser_converter::csv_format::CSVRecords;

fn main() {
    let input_path = Path::new("records_example.csv");
    let output_path = Path::new("res_check_csv.txt");

    let mut file = File::open(input_path).unwrap();
    let records = CSVRecords::from_read(&mut file).unwrap();

    let mut writer = BufWriter::new(File::create(output_path).unwrap());
    TXTRecords::write_to(records.as_records(), &mut writer).unwrap();
}
