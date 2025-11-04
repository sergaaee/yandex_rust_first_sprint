use parser_converter::Converter;
use parser_converter::bin_format::BinRecords;
use parser_converter::txt_format::TXTRecords;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let input_path = Path::new("res.txt");
    let output_path = Path::new("res_check.txt");

    let mut file = File::open(input_path).unwrap();
    // if let Ok(records) = BinRecords::from_read(&mut file) {
    //     for record in &records.records {
    //         println!("{:?}", record);
    //     }
    // }
    let records = TXTRecords::from_read(&mut file).unwrap();

    let mut writer = BufWriter::new(File::create(output_path).unwrap());
    TXTRecords::write_to(records.as_records(), &mut writer).unwrap();
}
