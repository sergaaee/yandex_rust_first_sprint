use crate::{Converter, Record, TxStatus, TxType};
use std::io::{Read, Write};

pub struct CSVRecords {
    pub records: Vec<Record>,
}

impl Converter for CSVRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, String> {
        let mut s = String::new();
        r.read_to_string(&mut s).map_err(|e| e.to_string())?;

        let mut records = Vec::new();

        // ожидаем заголовок:
        // TX_TYPE,STATUS,TO_USER_ID,FROM_USER_ID,TIMESTAMP,DESCRIPTION,TX_ID,AMOUNT
        let mut lines = s.lines();
        let header = lines
            .next()
            .ok_or("Пустой CSV-файл")?
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        for (line_num, line) in lines.enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let values: Vec<String> = line
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();

            if values.len() != header.len() {
                return Err(format!(
                    "Ошибка в строке {}: ожидалось {} столбцов, найдено {}",
                    line_num + 2,
                    header.len(),
                    values.len()
                ));
            }

            let mut map = std::collections::HashMap::new();
            for (k, v) in header.iter().zip(values.iter()) {
                map.insert(k.clone(), v.clone());
            }

            let record = parse_record(&map)?;
            records.push(record);
        }

        Ok(CSVRecords { records })
    }

    fn write_to<W: Write>(records: &Vec<Record>, writer: &mut W) -> Result<(), String> {
        writeln!(
            writer,
            "TX_TYPE,STATUS,TO_USER_ID,FROM_USER_ID,TIMESTAMP,DESCRIPTION,TX_ID,AMOUNT"
        )
            .map_err(|e| e.to_string())?;

        for rec in records {
            writeln!(
                writer,
                "{:?},{:?},{},{},{},{:?},{},{}",
                rec.tx_type,
                rec.tx_status,
                rec.to_user_id,
                rec.from_user_id,
                rec.timestamp,
                rec.description,
                rec.tx_id,
                rec.amount
            )
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn as_records(&self) -> &Vec<Record> {
        &self.records
    }
}

fn parse_record(map: &std::collections::HashMap<String, String>) -> Result<Record, String> {
    fn get<T: std::str::FromStr>(map: &std::collections::HashMap<String, String>, key: &str) -> Result<T, String> {
        map.get(key)
            .ok_or_else(|| format!("нет {}", key))?
            .parse::<T>()
            .map_err(|_| format!("ошибка парсинга {}", key))
    }

    let tx_type = match map.get("TX_TYPE").map(|s| s.as_str()) {
        Some("DEPOSIT") => TxType::DEPOSIT,
        Some("WITHDRAWAL") => TxType::WITHDRAWAL,
        Some("TRANSFER") => TxType::TRANSFER,
        Some(v) if v.eq_ignore_ascii_case("deposit") => TxType::DEPOSIT,
        Some(v) if v.eq_ignore_ascii_case("withdrawal") => TxType::WITHDRAWAL,
        Some(v) if v.eq_ignore_ascii_case("transfer") => TxType::TRANSFER,
        _ => return Err("неизвестный TX_TYPE".into()),
    };

    let tx_status = match map.get("STATUS").map(|s| s.as_str()) {
        Some("FAILURE") => TxStatus::FAILURE,
        Some("PENDING") => TxStatus::PENDING,
        Some("SUCCESS") => TxStatus::SUCCESS,
        Some(v) if v.eq_ignore_ascii_case("failure") => TxStatus::FAILURE,
        Some(v) if v.eq_ignore_ascii_case("pending") => TxStatus::PENDING,
        Some(v) if v.eq_ignore_ascii_case("success") => TxStatus::SUCCESS,
        _ => return Err("неизвестный STATUS".into()),
    };

    Ok(Record {
        tx_type,
        tx_status,
        to_user_id: get(map, "TO_USER_ID")?,
        from_user_id: get(map, "FROM_USER_ID")?,
        timestamp: get(map, "TIMESTAMP")?,
        description: map
            .get("DESCRIPTION")
            .cloned()
            .unwrap_or_default(),
        tx_id: get(map, "TX_ID")?,
        amount: get(map, "AMOUNT")?,
    })
}
