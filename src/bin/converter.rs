use clap::Parser;
use parser_converter::{
    Converter, bin_format::BinRecords, csv_format::CSVRecords, txt_format::TXTRecords,
};
use std::{
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};
use parser_converter::errors::ConvertingError;

#[derive(Parser, Debug)]
#[command(name = "ypbank_converter")]
#[command(about = "Converts YPBank transaction records between formats")]
struct Args {
    /// Input file path
    #[arg(long)]
    input: PathBuf,

    /// Output format: bin, txt, csv
    #[arg(long)]
    output_format: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let input_format = detect_format(&args.input).unwrap_or_else(|| "unknown".to_string());

    if input_format == args.output_format {
        eprintln!("❌ Input and output formats are the same — nothing to convert.");
        std::process::exit(1);
    }

    let mut file = File::open(&args.input)?;
    let mut output_path = PathBuf::new();
    output_path.push(&args.input);
    output_path.set_extension(&args.output_format);

    // читаем входной формат и записываем в новый
    match (input_format.as_str(), args.output_format.as_str()) {
        ("bin", "txt") => convert::<BinRecords, TXTRecords>(&mut file, &output_path).unwrap(),
        ("bin", "csv") => convert::<BinRecords, CSVRecords>(&mut file, &output_path).unwrap(),

        ("txt", "bin") => convert::<TXTRecords, BinRecords>(&mut file, &output_path).unwrap(),
        ("txt", "csv") => convert::<TXTRecords, CSVRecords>(&mut file, &output_path).unwrap(),

        ("csv", "bin") => convert::<CSVRecords, BinRecords>(&mut file, &output_path).unwrap(),
        ("csv", "txt") => convert::<CSVRecords, TXTRecords>(&mut file, &output_path).unwrap(),

        _ => {
            eprintln!(
                "❌ Unknown format combination: {} → {}",
                input_format, args.output_format
            );
            std::process::exit(1);
        }
    }

    println!("✅ Конвертация завершена: {:?}", output_path);
    Ok(())
}

fn convert<From, To>(reader: &mut File, output_path: &PathBuf) -> Result<(), ConvertingError>
where
    From: Converter,
    To: Converter,
{
    let records = From::from_read(reader).unwrap();
    let mut writer = BufWriter::new(File::create(output_path).map_err(|e| ConvertingError::IoError(e))?);
    To::write_to(records.as_records(), &mut writer)
}

fn detect_format(path: &PathBuf) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}
