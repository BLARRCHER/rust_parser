use crate::error::{ParseError, Result};
use crate::operation::{Operation, OperationStatus, OperationType};
use std::collections::HashSet;
use std::io::{Read, Write};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E]; // магическое 'YPBN'

/// Походили по бинарнику и собираем операцию по отступам
pub fn parse_operation<R: Read>(reader: &mut R) -> Result<Operation> {
    // Read and verify MAGIC
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;

    if magic != MAGIC {
        return Err(ParseError::InvalidMagic);
    }

    // Read RECORD_SIZE
    let mut size_buf = [0u8; 4];
    reader.read_exact(&mut size_buf)?;
    let _record_size = u32::from_be_bytes(size_buf);

    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    let tx_id = u64::from_be_bytes(buf);

    let mut type_buf = [0u8; 1];
    reader.read_exact(&mut type_buf)?;
    let tx_type = OperationType::from_u8(type_buf[0])?;

    reader.read_exact(&mut buf)?;
    let from_user_id = u64::from_be_bytes(buf);

    reader.read_exact(&mut buf)?;
    let to_user_id = u64::from_be_bytes(buf);

    reader.read_exact(&mut buf)?;
    let amount = i64::from_be_bytes(buf);

    reader.read_exact(&mut buf)?;
    let timestamp = u64::from_be_bytes(buf);

    reader.read_exact(&mut type_buf)?;
    let status = OperationStatus::from_u8(type_buf[0])?;

    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let desc_len = u32::from_be_bytes(len_buf) as usize;

    let mut desc_bytes = vec![0u8; desc_len];
    reader.read_exact(&mut desc_bytes)?;
    let raw_description = String::from_utf8(desc_bytes).map_err(|e| ParseError::InvalidField {
        field: "DESCRIPTION".to_string(),
        reason: format!("Invalid UTF-8: {}", e),
    })?;

    // Чистим ковычки
    let description = normalize_description(&raw_description);

    let operation = Operation {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    };

    operation.validate()?;
    Ok(operation)
}

/// Для лишн ковычек
fn normalize_description(s: &str) -> String {
    let trimmed = s.trim();

    let unquoted = if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };

    unescape_string(unquoted)
}

/// Для лишн ковычек
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    '"' => {
                        result.push('"');
                        chars.next();
                    }
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    'n' => {
                        result.push('\n');
                        chars.next();
                    }
                    't' => {
                        result.push('\t');
                        chars.next();
                    }
                    'r' => {
                        result.push('\r');
                        chars.next();
                    }
                    _ => {
                        result.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Запись экзм операции в бинарник
pub fn write_operation<W: Write>(writer: &mut W, operation: &Operation) -> Result<()> {
    operation.validate()?;

    // Вот хз я пишу без ковычек и эскейпинга
    let desc_bytes = operation.description.as_bytes();
    let desc_len = desc_bytes.len() as u32;

    // Тип пэддинг)
    let record_size: u32 = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + desc_len;

    writer.write_all(&MAGIC)?;
    writer.write_all(&record_size.to_be_bytes())?;
    writer.write_all(&operation.tx_id.to_be_bytes())?;
    writer.write_all(&[operation.tx_type.to_u8()])?;
    writer.write_all(&operation.from_user_id.to_be_bytes())?;
    writer.write_all(&operation.to_user_id.to_be_bytes())?;
    writer.write_all(&operation.amount.to_be_bytes())?;
    writer.write_all(&operation.timestamp.to_be_bytes())?;
    writer.write_all(&[operation.status.to_u8()])?;
    writer.write_all(&desc_len.to_be_bytes())?;
    writer.write_all(desc_bytes)?;

    Ok(())
}

/// Ходим по бинарнику, разбиваем по блокам и парсим операцию
pub fn parse_all<R: Read>(mut reader: R) -> Result<HashSet<Operation>> {
    let mut operations = HashSet::new();

    loop {
        match parse_operation(&mut reader) {
            Ok(op) => {
                operations.insert(op);
            }
            Err(ParseError::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }

    Ok(operations)
}

/// Итерируемся по операциям и записываем в бинарник
pub fn write_all<W: Write>(mut writer: W, operations: &HashSet<Operation>) -> Result<()> {
    for operation in operations {
        write_operation(&mut writer, operation)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation::{Operation, OperationStatus, OperationType};
    use std::io::Cursor;

    #[test]
    fn test_unescape_string() {
        assert_eq!(unescape_string(r#"Record number 1"#), "Record number 1");
        assert_eq!(
            unescape_string(r#"\"Record number 1\""#),
            r#""Record number 1""#
        );
        assert_eq!(unescape_string(r#"Line1\nLine2"#), "Line1\nLine2");
        assert_eq!(unescape_string(r#"Tab\there"#), "Tab\there");
        assert_eq!(unescape_string(r#"Backslash\\"#), r#"Backslash\"#);
    }

    #[test]
    fn test_normalize_description() {
        assert_eq!(normalize_description(r#""Нормализуй 1""#), "Нормализуй 1");
        assert_eq!(
            normalize_description(r#""\"Нормализуй 1\"""#),
            r#""Нормализуй 1""#
        );
        assert_eq!(normalize_description("Нормализуй 1"), "Нормализуй 1");
        assert_eq!(normalize_description(r#"  "trimmed"  "#), "trimmed");
    }

    #[test]
    fn test_round_trip_simple() {
        let op = Operation {
            tx_id: 12345,
            tx_type: OperationType::Deposit,
            from_user_id: 0,
            to_user_id: 67890,
            amount: 1000,
            timestamp: 1633036860000,
            status: OperationStatus::Success,
            description: "Simple".to_string(),
        };

        let mut buf = Vec::new();
        write_operation(&mut buf, &op).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = parse_operation(&mut cursor).unwrap();

        assert_eq!(op, parsed);
        assert_eq!(parsed.description, "Simple");
    }

    #[test]
    fn test_parse_escaped_description() {
        let op_with_escaped = Operation {
            tx_id: 1000000000000000,
            tx_type: OperationType::Deposit,
            from_user_id: 0,
            to_user_id: 9223372036854775807,
            amount: 100,
            timestamp: 1633036860000,
            status: OperationStatus::Failure,
            description: r#"\"Лишн ковычк 1\""#.to_string(),
        };

        let mut buf = Vec::new();
        write_operation(&mut buf, &op_with_escaped).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = parse_operation(&mut cursor).unwrap();

        assert_eq!(parsed.description, r#""Лишн ковычк 1""#);
    }

    #[test]
    fn test_round_trip_with_quotes() {
        let op = Operation {
            tx_id: 12345,
            tx_type: OperationType::Deposit,
            from_user_id: 0,
            to_user_id: 67890,
            amount: 1000,
            timestamp: 1633036860000,
            status: OperationStatus::Success,
            description: r#"Ковычк должны остаться "quotes""#.to_string(),
        };

        let mut buf = Vec::new();
        write_operation(&mut buf, &op).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = parse_operation(&mut cursor).unwrap();

        assert_eq!(op, parsed);
        assert_eq!(parsed.description, r#"Ковычк должны остаться "quotes""#);
    }

    #[test]
    fn test_round_trip_unicode() {
        let op = Operation {
            tx_id: 12345,
            tx_type: OperationType::Deposit,
            from_user_id: 0,
            to_user_id: 67890,
            amount: 1000,
            timestamp: 1633036860000,
            status: OperationStatus::Success,
            description: "Ну по-русски 🎉".to_string(),
        };

        let mut buf = Vec::new();
        write_operation(&mut buf, &op).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = parse_operation(&mut cursor).unwrap();

        assert_eq!(op, parsed);
        assert_eq!(parsed.description, "Ну по-русски 🎉");
    }

    #[test]
    fn test_empty_description() {
        let op = Operation {
            tx_id: 12345,
            tx_type: OperationType::Deposit,
            from_user_id: 0,
            to_user_id: 67890,
            amount: 1000,
            timestamp: 1633036860000,
            status: OperationStatus::Success,
            description: String::new(),
        };

        let mut buf = Vec::new();
        write_operation(&mut buf, &op).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = parse_operation(&mut cursor).unwrap();

        assert_eq!(op, parsed);
        assert_eq!(parsed.description, "");
    }
}
