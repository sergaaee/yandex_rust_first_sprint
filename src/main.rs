use parser_converter::Converter;
use parser_converter::bin_format::BinRecords;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let input_path = Path::new("res.bin");
    let output_path = Path::new("res.bin");

    let mut file = File::open(input_path).unwrap();
    if let Ok(records) = BinRecords::from_read(&mut file) {
        for record in &records.records {
            println!("{:?}", record);
        }
    }
    // let mut records = BinRecords::from_read(&mut file).unwrap();
    //
    // let mut writer = BufWriter::new(File::create(output_path).unwrap());
    // BinRecords::write_to(&mut records, &mut writer).unwrap();
}
