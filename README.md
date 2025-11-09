# 📦 YPBank Parser & Converter Library

YPBank is a Rust library providing **unified transaction parsing and conversion** between multiple data formats — Binary (`.bin`), Text (`.txt`), and CSV (`.csv`).
It’s designed for **financial transaction systems**, allowing consistent read/write operations across heterogeneous file formats.

---

## ✨ Features

* ✅ Unified data structure for all formats
* 🔄 Convert between `.bin`, `.txt`, and `.csv` formats easily
* 🧩 Pluggable `Converter` trait for extending formats
* ⚡ High-performance binary IO
* 💬 Human-readable text and CSV formats
* 🧠 Strong type safety for transactions (`TxType`, `TxStatus`)

---

## 📖 Example Usage

### Reading Transactions

```rust
use ypbank::bin_format::BinRecords;
use ypbank::Converter;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("transactions.bin")?;
    let records = BinRecords::from_read(&mut file)?;
    for record in records.as_records() {
        println!("{:?}", record);
    }
    Ok(())
}
```

---

### Writing Transactions

```rust
use ypbank::{Record, TxType, TxStatus, Converter};
use ypbank::txt_format::TXTRecords;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = vec![
        Record {
            tx_type: TxType::DEPOSIT,
            tx_status: TxStatus::SUCCESS,
            to_user_id: 1001,
            from_user_id: 0,
            timestamp: 1_731_000_000_000,
            description: "Initial deposit".to_string(),
            tx_id: 12345,
            amount: 500_00,
        },
    ];

    let mut output = File::create("output.txt")?;
    TXTRecords::write_to(&records, &mut output)?;
    Ok(())
}
```

---

## 🧱 Core Structures

### `Record`

Represents a single transaction entry with metadata:

```rust
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
```

---

### `Converter` Trait

Defines a unified interface for reading and writing data:

```rust
pub trait Converter {
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, ParsingError>
    where
        Self: Sized;

    fn write_to<W: std::io::Write>(
        records: &[Record],
        writer: &mut W,
    ) -> Result<(), ConvertingError>;

    fn as_records(&self) -> &[Record];
}
```

Implement this trait to add support for new formats (e.g., JSON, XML).

---

## 📂 Supported Formats

| Format | Description                                 | File Extension |
| :----- | :------------------------------------------ | :------------- |
| `Bin`  | Binary format optimized for compact storage | `.bin`         |
| `Txt`  | Plain text format for readability           | `.txt`         |
| `Csv`  | Comma-separated values for spreadsheets     | `.csv`         |

---

## ⚙️ Constants

* `MIN_FIXED_SIZE = 46` — Minimum number of bytes required for the fixed part of a binary record (excluding description).

---

## 🧩 Error Handling

Two error types are used:

* `ParsingError` — for errors while reading/parsing data.
* `ConvertingError` — for errors during conversion or writing.

---

## 🧠 Example: Adding a New Format

You can add a new converter by implementing the `Converter` trait:

```rust
pub struct JsonRecords {
    records: Vec<Record>,
}

impl Converter for JsonRecords {
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, ParsingError> {
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        let records: Vec<Record> = serde_json::from_str(&buf)?;
        Ok(JsonRecords { records })
    }

    fn write_to<W: std::io::Write>(
        records: &[Record],
        writer: &mut W,
    ) -> Result<(), ConvertingError> {
        let json = serde_json::to_string_pretty(records)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn as_records(&self) -> &[Record] {
        &self.records
    }
}
```

---

## 🧰 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ypbank = { path = "./ypbank" }
clap = "4"
```

---

## 🏁 License

MIT © 2025 sergaaee

---

## 💬 Contributing

Contributions are welcome!
Please open a Pull Request or file an Issue on GitHub.
