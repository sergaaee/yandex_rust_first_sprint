use crate::{Converter, Record, TxStatus, TxType};
use std::collections::HashMap;
use std::io::{Read, Write};

pub struct TXTRecords {
    pub records: Vec<Record>,
}

impl Converter for TXTRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, String> {
        let mut s = String::new();
        r.read_to_string(&mut s).map_err(|e| e.to_string())?;

        let mut records = Vec::new();
        let mut current = HashMap::<String, String>::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // начало новой записи
            if line.starts_with("# Record") {
                if !current.is_empty() {
                    records.push(parse_record(&current)?);
                    current.clear();
                }
            } else if let Some((k, v)) = line.split_once(':') {
                let key = k.trim().to_string();
                let mut value = v.trim().to_string();
                if value.starts_with('"') && value.ends_with('"') {
                    value = value[1..value.len() - 1].to_string();
                }
                current.insert(key, value);
            }
        }

        // последняя запись
        if !current.is_empty() {
            records.push(parse_record(&current)?);
        }

        Ok(TXTRecords { records })
    }

    fn write_to<W: Write>(records: &Vec<Record>, writer: &mut W) -> Result<(), String> {
        for (i, record) in records.iter().enumerate() {
            writeln!(writer, "# Record {} ({:?})", i + 1, record.tx_type).unwrap();
            writeln!(writer, "TX_TYPE: {:?}", record.tx_type).unwrap();
            writeln!(writer, "TO_USER_ID: {}", record.to_user_id).unwrap();
            writeln!(writer, "FROM_USER_ID: {}", record.from_user_id).unwrap();
            writeln!(writer, "TIMESTAMP: {}", record.timestamp).unwrap();
            writeln!(writer, "DESCRIPTION: {:?}", record.description).unwrap();
            writeln!(writer, "TX_ID: {}", record.tx_id).unwrap();
            writeln!(writer, "AMOUNT: {}", record.amount).unwrap();
            writeln!(writer, "STATUS: {:?}", record.tx_status).unwrap();
            writeln!(writer).unwrap();
        }
        Ok(())
    }

    fn as_records(&self) -> &Vec<Record> {
        &self.records
    }
}

fn parse_record(map: &HashMap<String, String>) -> Result<Record, String> {
    fn get<T: std::str::FromStr>(map: &HashMap<String, String>, key: &str) -> Result<T, String> {
        map.get(key)
            .ok_or_else(|| format!("нет {}", key))?
            .parse::<T>()
            .map_err(|_| format!("ошибка парсинга {}", key))
    }

    let tx_type = match map.get("TX_TYPE").map(|s| s.as_str()) {
        Some("DEPOSIT") => TxType::DEPOSIT,
        Some("WITHDRAWAL") => TxType::WITHDRAWAL,
        Some("TRANSFER") => TxType::TRANSFER,
        _ => return Err("неизвестный TX_TYPE".into()),
    };

    let tx_status = match map.get("STATUS").map(|s| s.as_str()) {
        Some("FAILURE") => TxStatus::FAILURE,
        Some("PENDING") => TxStatus::PENDING,
        Some("SUCCESS") => TxStatus::SUCCESS,
        _ => return Err("неизвестный STATUS".into()),
    };

    Ok(Record {
        tx_type,
        tx_status,
        to_user_id: get(map, "TO_USER_ID")?,
        from_user_id: get(map, "FROM_USER_ID")?,
        timestamp: get(map, "TIMESTAMP")?,
        description: map.get("DESCRIPTION").cloned().unwrap_or_default(),
        tx_id: get(map, "TX_ID")?,
        amount: get(map, "AMOUNT")?,
    })
}
