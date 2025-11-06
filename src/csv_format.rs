use crate::errors::{ConvertingError, ParsingError};
use crate::{Converter, Record, TxStatus, TxType};
use std::collections::HashMap;
use std::io::{Read, Write};

pub struct CSVRecords {
    pub records: Vec<Record>,
}

impl Converter for CSVRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, ParsingError> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;

        let mut records = Vec::new();

        // ожидаем заголовок:
        // TX_TYPE,STATUS,TO_USER_ID,FROM_USER_ID,TIMESTAMP,DESCRIPTION,TX_ID,AMOUNT
        let mut lines = s.lines();
        let header = lines
            .next()
            .ok_or(ParsingError::EmptyFile)?
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
                return Err(ParsingError::WrongColumnCount(
                    line_num + 2,
                    header.len(),
                    values.len(),
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

    fn write_to<W: Write>(records: &[Record], writer: &mut W) -> Result<(), ConvertingError> {
        writeln!(
            writer,
            "TX_TYPE,STATUS,TO_USER_ID,FROM_USER_ID,TIMESTAMP,DESCRIPTION,TX_ID,AMOUNT"
        )?;

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
            )?
        }

        Ok(())
    }

    fn as_records(&self) -> &[Record] {
        &self.records
    }
}

fn parse_record(map: &HashMap<String, String>) -> Result<Record, ParsingError> {
    fn get<T: std::str::FromStr>(
        map: &HashMap<String, String>,
        key: &str,
    ) -> Result<T, ParsingError> {
        let value = map
            .get(key)
            .ok_or_else(|| ParsingError::MissingKey(key.to_string()))?;

        value
            .parse::<T>()
            .map_err(|_| ParsingError::WrongKey(key.to_string()))
    }

    let tx_type = match map.get("TX_TYPE").map(|s| s.as_str()) {
        Some("DEPOSIT") => TxType::DEPOSIT,
        Some("WITHDRAWAL") => TxType::WITHDRAWAL,
        Some("TRANSFER") => TxType::TRANSFER,
        Some(v) if v.eq_ignore_ascii_case("deposit") => TxType::DEPOSIT,
        Some(v) if v.eq_ignore_ascii_case("withdrawal") => TxType::WITHDRAWAL,
        Some(v) if v.eq_ignore_ascii_case("transfer") => TxType::TRANSFER,
        _ => return Err(ParsingError::WrongTxType),
    };

    let tx_status = match map.get("STATUS").map(|s| s.as_str()) {
        Some("FAILURE") => TxStatus::FAILURE,
        Some("PENDING") => TxStatus::PENDING,
        Some("SUCCESS") => TxStatus::SUCCESS,
        Some(v) if v.eq_ignore_ascii_case("failure") => TxStatus::FAILURE,
        Some(v) if v.eq_ignore_ascii_case("pending") => TxStatus::PENDING,
        Some(v) if v.eq_ignore_ascii_case("success") => TxStatus::SUCCESS,
        _ => return Err(ParsingError::WrongStatusType),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_records() -> Vec<Record> {
        vec![
            Record {
                tx_type: TxType::DEPOSIT,
                tx_status: TxStatus::SUCCESS,
                to_user_id: 100,
                from_user_id: 0,
                timestamp: 1700000000,
                description: "Пополнение счёта".to_string(),
                tx_id: 1,
                amount: 5000,
            },
            Record {
                tx_type: TxType::TRANSFER,
                tx_status: TxStatus::PENDING,
                to_user_id: 200,
                from_user_id: 100,
                timestamp: 1700000100,
                description: "Перевод другу".to_string(),
                tx_id: 2,
                amount: 1500,
            },
        ]
    }

    #[test]
    fn test_csv_write_and_read_back() {
        let records = sample_records();

        // Пишем CSV в буфер
        let mut buf = Cursor::new(Vec::new());
        CSVRecords::write_to(&records, &mut buf).unwrap();

        // Читаем CSV обратно
        buf.set_position(0);
        let parsed = CSVRecords::from_read(&mut buf).unwrap();

        // Проверяем, что всё совпадает
        assert_eq!(parsed.records.len(), 2);
        assert_eq!(parsed.records[0].tx_id, 1);
        assert_eq!(parsed.records[0].tx_type, TxType::DEPOSIT);
        assert_eq!(parsed.records[0].tx_status, TxStatus::SUCCESS);
        assert_eq!(parsed.records[0].description, "Пополнение счёта");
        assert_eq!(parsed.records[1].amount, 1500);
        assert_eq!(parsed.records[1].tx_status, TxStatus::PENDING);
    }

    #[test]
    fn test_csv_from_read_with_invalid_header() {
        let bad_csv = "WRONG_HEADER\nsomething,else\n";
        let mut cursor = Cursor::new(bad_csv.as_bytes());
        let result = CSVRecords::from_read(&mut cursor);
        assert!(
            result.is_err(),
            "ожидалась ошибка при неверном CSV-заголовке"
        );
    }

    #[test]
    fn test_csv_from_read_with_empty_file() {
        let mut cursor = Cursor::new(b"");
        let result = CSVRecords::from_read(&mut cursor);
        assert!(result.is_err());
    }
}
