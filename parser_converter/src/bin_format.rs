use crate::errors::{ConvertingError, ParsingError};
use crate::{Converter, MIN_FIXED_SIZE, Record, TxStatus, TxType};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

/// Records structure for .bin format
pub struct BinRecords {
    /// Records
    pub records: Vec<Record>,
}

impl Converter for BinRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, ParsingError> {
        let mut records = Vec::new();

        loop {
            let mut magic = [0u8; 4];
            if r.read_exact(&mut magic).is_err() {
                break; // EOF
            }

            if &magic != b"YPBN" {
                return Err(ParsingError::InvalidMagicHeader);
            }

            let record_size = match r.read_u32::<BigEndian>() {
                Ok(v) => Ok(v),
                Err(_) => Err(ParsingError::WrongRecordData),
            }?;

            if record_size < MIN_FIXED_SIZE {
                return Err(ParsingError::WrongRecordData);
            }

            let mut body = vec![0u8; record_size as usize];
            if r.read_exact(&mut body).is_err() {
                return Err(ParsingError::WrongRecordData);
            };

            let mut cursor = std::io::Cursor::new(body);

            let tx_id = cursor.read_u64::<BigEndian>()?;
            let tx_type = match cursor.read_u8()? {
                0 => TxType::DEPOSIT,
                1 => TxType::TRANSFER,
                2 => TxType::WITHDRAWAL,
                _ => return Err(ParsingError::WrongTxType),
            };

            let from_user_id = cursor.read_u64::<BigEndian>()?;
            let to_user_id = cursor.read_u64::<BigEndian>()?;
            let amount = cursor.read_u64::<BigEndian>()?;
            let timestamp = cursor.read_u64::<BigEndian>()?;

            let tx_status = match cursor.read_u8()? {
                0 => TxStatus::SUCCESS,
                1 => TxStatus::FAILURE,
                2 => TxStatus::PENDING,
                _ => return Err(ParsingError::WrongStatusType),
            };

            let desc_len = cursor.read_u32::<BigEndian>()?;

            // Проверка, что desc_len корректно укладывается в record_size
            let expected_total = MIN_FIXED_SIZE + desc_len;
            if expected_total > record_size {
                return Err(ParsingError::WrongRecordData);
            }

            let mut desc_buf = vec![0u8; desc_len as usize];
            cursor.read_exact(&mut desc_buf)?;
            let description = String::from_utf8(desc_buf)?.to_string().replace("\"", "");

            records.push(Record {
                tx_type,
                tx_status,
                to_user_id,
                from_user_id,
                timestamp,
                description,
                tx_id,
                amount,
            });
        }

        Ok(BinRecords { records })
    }

    fn write_to<W: Write>(records: &[Record], w: &mut W) -> Result<(), ConvertingError> {
        for record in records {
            let mut body = Vec::new();

            // === Поля тела ===
            body.write_u64::<BigEndian>(record.tx_id)?;

            body.write_u8(match record.tx_type {
                TxType::DEPOSIT => 0,
                TxType::TRANSFER => 1,
                TxType::WITHDRAWAL => 2,
            })?;

            body.write_u64::<BigEndian>(record.from_user_id)?;
            body.write_u64::<BigEndian>(record.to_user_id)?;
            body.write_i64::<BigEndian>(record.amount as i64)?;
            body.write_u64::<BigEndian>(record.timestamp)?;

            body.write_u8(match record.tx_status {
                TxStatus::SUCCESS => 0,
                TxStatus::FAILURE => 1,
                TxStatus::PENDING => 2,
            })?;

            let desc_bytes = record.description.as_bytes();
            body.write_u32::<BigEndian>(desc_bytes.len() as u32)?;
            body.write_all(desc_bytes)?;

            // === Заголовок ===
            w.write_all(b"YPBN")?;
            w.write_u32::<BigEndian>(body.len() as u32)?;
            w.write_all(&body)?;
        }

        Ok(())
    }

    fn as_records(&self) -> &[Record] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_record(id: u64, desc: &str) -> Record {
        Record {
            tx_type: TxType::DEPOSIT,
            tx_status: TxStatus::SUCCESS,
            to_user_id: 1000,
            from_user_id: 0,
            timestamp: 1_634_000_000_000,
            description: desc.to_string(),
            tx_id: id,
            amount: 500,
        }
    }

    #[test]
    fn test_write_and_read_single_record() {
        let record = sample_record(42, "Single record");
        let records = vec![record];

        // Пишем в память
        let mut buf = Vec::new();
        BinRecords::write_to(&records, &mut buf).unwrap();

        // Читаем из памяти
        let mut cursor = Cursor::new(buf);
        let parsed = BinRecords::from_read(&mut cursor).unwrap();

        assert_eq!(parsed.records.len(), 1);
        let rec = &parsed.records[0];
        assert_eq!(rec.tx_id, 42);
        assert_eq!(rec.description, "Single record");
        assert_eq!(rec.tx_type, TxType::DEPOSIT);
        assert_eq!(rec.tx_status, TxStatus::SUCCESS);
        assert_eq!(rec.amount, 500);
    }

    #[test]
    fn test_multiple_records_roundtrip() {
        let input_records = vec![
            sample_record(1, "First"),
            sample_record(2, "Second"),
            sample_record(3, "Third"),
        ];

        let mut buf = Vec::new();
        BinRecords::write_to(&input_records, &mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let output = BinRecords::from_read(&mut cursor).unwrap();

        assert_eq!(input_records.len(), output.records.len());
        for (a, b) in input_records.iter().zip(output.records.iter()) {
            assert_eq!(a.tx_id, b.tx_id);
            assert_eq!(a.description, b.description);
            assert_eq!(a.amount, b.amount);
        }
    }

    #[test]
    fn test_invalid_magic_header() {
        // записываем неправильный заголовок
        let mut buf = Vec::new();
        buf.extend_from_slice(b"XXXX");
        buf.extend_from_slice(&[0, 0, 0, 0]);

        let mut cursor = Cursor::new(buf);
        let result = BinRecords::from_read(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_data_returns_error() {
        // создаём корректную запись и обрываем на середине
        let record = sample_record(10, "Incomplete");
        let mut buf = Vec::new();
        BinRecords::write_to(&[record], &mut buf).unwrap();

        // обрезаем половину данных
        let cutoff = buf.len() / 2;
        let mut truncated = Cursor::new(&buf[..cutoff]);

        // вызываем парсер
        let result = BinRecords::from_read(&mut truncated);

        // ожидаем ошибку
        assert!(result.is_err(), "Expected error for truncated binary data");

        // проверяем конкретный тип ошибки
        if let Err(ParsingError::WrongRecordData) = result {
            // ок, всё верно
        } else {
            panic!("Expected ParsingError::WrongRecordData for truncated input");
        }
    }
}
