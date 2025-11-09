use crate::errors::{ConvertingError, ParsingError};
use crate::{Converter, Record, TxStatus, TxType};
use std::collections::HashMap;
use std::io::{Read, Write};

/// Records structure for .bin format
pub struct TXTRecords {
    /// Records
    pub records: Vec<Record>,
}

impl Converter for TXTRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, ParsingError> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;

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

    fn write_to<W: Write>(records: &[Record], writer: &mut W) -> Result<(), ConvertingError> {
        for (i, record) in records.iter().enumerate() {
            writeln!(writer, "# Record {} ({:?})", i + 1, record.tx_type)?;
            writeln!(writer, "TX_TYPE: {:?}", record.tx_type)?;
            writeln!(writer, "TO_USER_ID: {}", record.to_user_id)?;
            writeln!(writer, "FROM_USER_ID: {}", record.from_user_id)?;
            writeln!(writer, "TIMESTAMP: {}", record.timestamp)?;
            writeln!(writer, "DESCRIPTION: {:?}", record.description)?;
            writeln!(writer, "TX_ID: {}", record.tx_id)?;
            writeln!(writer, "AMOUNT: {}", record.amount)?;
            writeln!(writer, "STATUS: {:?}", record.tx_status)?;
            writeln!(writer)?;
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
        _ => return Err(ParsingError::WrongTxType),
    };

    let tx_status = match map.get("STATUS").map(|s| s.as_str()) {
        Some("FAILURE") => TxStatus::FAILURE,
        Some("PENDING") => TxStatus::PENDING,
        Some("SUCCESS") => TxStatus::SUCCESS,
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
                to_user_id: 111,
                from_user_id: 0,
                timestamp: 1700000000,
                description: "Record 1".to_string(),
                tx_id: 1,
                amount: 5000,
            },
            Record {
                tx_type: TxType::TRANSFER,
                tx_status: TxStatus::PENDING,
                to_user_id: 222,
                from_user_id: 111,
                timestamp: 1700000500,
                description: "Record 2".to_string(),
                tx_id: 2,
                amount: 1500,
            },
        ]
    }

    #[test]
    fn test_txt_write_and_read_back() {
        let records = sample_records();

        // Пишем в текст
        let mut buf = Cursor::new(Vec::new());
        TXTRecords::write_to(&records, &mut buf).unwrap();

        // Читаем обратно
        buf.set_position(0);
        let parsed = TXTRecords::from_read(&mut buf).unwrap();

        // Проверяем количество и значения
        assert_eq!(parsed.records.len(), 2);
        assert_eq!(parsed.records[0].tx_id, 1);
        assert_eq!(parsed.records[0].tx_type, TxType::DEPOSIT);
        assert_eq!(parsed.records[0].tx_status, TxStatus::SUCCESS);
        assert_eq!(parsed.records[0].description, "Record 1");

        assert_eq!(parsed.records[1].tx_id, 2);
        assert_eq!(parsed.records[1].tx_type, TxType::TRANSFER);
        assert_eq!(parsed.records[1].tx_status, TxStatus::PENDING);
        assert_eq!(parsed.records[1].description, "Record 2");
    }

    #[test]
    fn test_txt_from_read_with_missing_field() {
        let bad_txt = r#"
# Record 1 (DEPOSIT)
TX_TYPE: DEPOSIT
TO_USER_ID: 100
# FROM_USER_ID отсутствует
TIMESTAMP: 1700000000
DESCRIPTION: "Тест"
TX_ID: 10
AMOUNT: 1000
STATUS: SUCCESS
"#;
        let mut cursor = Cursor::new(bad_txt.as_bytes());
        let result = TXTRecords::from_read(&mut cursor);
        assert!(result.is_err(), "ожидалась ошибка при отсутствии поля");
    }

    #[test]
    fn test_txt_from_read_with_empty_file() {
        let mut cursor = Cursor::new(b"");
        let result = TXTRecords::from_read(&mut cursor);
        // Пустой файл — просто без записей, но не ошибка
        assert!(result.is_ok());
        assert!(result.unwrap().records.is_empty());
    }

    #[test]
    fn test_txt_from_read_with_invalid_type() {
        let invalid = r#"
# Record 1 (INVALID)
TX_TYPE: UNKNOWN
TO_USER_ID: 1
FROM_USER_ID: 2
TIMESTAMP: 123
DESCRIPTION: "desc"
TX_ID: 5
AMOUNT: 99
STATUS: SUCCESS
"#;
        let mut cursor = Cursor::new(invalid.as_bytes());
        let result = TXTRecords::from_read(&mut cursor);
        assert!(
            result.is_err(),
            "должна быть ошибка для неизвестного TX_TYPE"
        );
    }

    #[test]
    fn test_txt_from_read_with_invalid_status() {
        let invalid = r#"
# Record 1 (DEPOSIT)
TX_TYPE: DEPOSIT
TO_USER_ID: 1
FROM_USER_ID: 2
TIMESTAMP: 123
DESCRIPTION: "desc"
TX_ID: 5
AMOUNT: 99
STATUS: WTF
"#;
        let mut cursor = Cursor::new(invalid.as_bytes());
        let result = TXTRecords::from_read(&mut cursor);
        assert!(
            result.is_err(),
            "должна быть ошибка для неизвестного STATUS"
        );
    }
}
