use crate::{Converter, Record, TxStatus, TxType};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

pub struct BinRecords {
    pub records: Vec<Record>,
}

impl Converter for BinRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, String> {
        let mut records = Vec::new();

        loop {
            let mut magic = [0u8; 4];
            if r.read_exact(&mut magic).is_err() {
                break; // EOF
            }

            if &magic != b"YPBN" {
                return Err("Invalid MAGIC header".into());
            }

            let record_size = match r.read_u32::<BigEndian>() {
                Ok(v) => v,
                Err(_) => break,
            };

            let mut body = vec![0u8; record_size as usize];
            if r.read_exact(&mut body).is_err() {
                break;
            }

            let mut cursor = std::io::Cursor::new(body);

            let tx_id = cursor.read_u64::<BigEndian>().map_err(|e| e.to_string())?;
            let tx_type = match cursor.read_u8().map_err(|e| e.to_string())? {
                0 => TxType::DEPOSIT,
                1 => TxType::TRANSFER,
                2 => TxType::WITHDRAWAL,
                n => return Err(format!("Unknown TX_TYPE: {}", n)),
            };

            let from_user_id = cursor.read_u64::<BigEndian>().map_err(|e| e.to_string())?;
            let to_user_id = cursor.read_u64::<BigEndian>().map_err(|e| e.to_string())?;
            let amount = cursor.read_u64::<BigEndian>().map_err(|e| e.to_string())?;
            let timestamp = cursor.read_u64::<BigEndian>().map_err(|e| e.to_string())?;

            let tx_status = match cursor.read_u8().map_err(|e| e.to_string())? {
                0 => TxStatus::SUCCESS,
                1 => TxStatus::FAILURE,
                2 => TxStatus::PENDING,
                n => return Err(format!("Unknown TX_STATUS: {}", n)),
            };

            let desc_len = cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?;
            let mut desc_buf = vec![0u8; desc_len as usize];
            cursor
                .read_exact(&mut desc_buf)
                .map_err(|e| e.to_string())?;
            let description = String::from_utf8_lossy(&desc_buf)
                .to_string()
                .replace("\"", "");

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

    fn write_to<W: Write>(records: &Vec<Record>, w: &mut W) -> Result<(), String> {
        for record in records {
            let mut body = Vec::new();

            // === Поля тела ===
            body.write_u64::<BigEndian>(record.tx_id)
                .map_err(|e| e.to_string())?;

            body.write_u8(match record.tx_type {
                TxType::DEPOSIT => 0,
                TxType::TRANSFER => 1,
                TxType::WITHDRAWAL => 2,
            })
            .map_err(|e| e.to_string())?;

            body.write_u64::<BigEndian>(record.from_user_id)
                .map_err(|e| e.to_string())?;
            body.write_u64::<BigEndian>(record.to_user_id)
                .map_err(|e| e.to_string())?;
            body.write_i64::<BigEndian>(record.amount as i64)
                .map_err(|e| e.to_string())?;
            body.write_u64::<BigEndian>(record.timestamp)
                .map_err(|e| e.to_string())?;

            body.write_u8(match record.tx_status {
                TxStatus::SUCCESS => 0,
                TxStatus::FAILURE => 1,
                TxStatus::PENDING => 2,
            })
            .map_err(|e| e.to_string())?;

            let desc_bytes = record.description.as_bytes();
            body.write_u32::<BigEndian>(desc_bytes.len() as u32)
                .map_err(|e| e.to_string())?;
            body.write_all(desc_bytes).map_err(|e| e.to_string())?;

            // === Заголовок ===
            w.write_all(b"YPBN").map_err(|e| e.to_string())?;
            w.write_u32::<BigEndian>(body.len() as u32)
                .map_err(|e| e.to_string())?;
            w.write_all(&body).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn as_records(&self) -> &Vec<Record> {
        &self.records
    }
}
