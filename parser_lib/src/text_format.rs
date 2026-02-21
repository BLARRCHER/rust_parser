use crate::error::{ParseError, Result};
use crate::operation::{Operation, OperationStatus, OperationType};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

/// Читаем с txt файла
pub fn parse_all<R: Read>(reader: R) -> Result<HashSet<Operation>> {
    let buf_reader = BufReader::new(reader);
    let lines = buf_reader.lines().peekable();
    let mut operations = HashSet::new();

    let mut current_record: HashMap<String, String> = HashMap::new();

    for line in lines {
        let line = line?;
        let trimmed = line.trim();

        // Скип комменты и пуст стр
        if trimmed.is_empty() || trimmed.starts_with('#') {
            // Если до пустой строки чтот читали то считаем что экз операции кончился
            if !current_record.is_empty() && trimmed.is_empty() {
                let operation = parse_record(&current_record)?;
                operation.validate()?;
                operations.insert(operation);
                current_record.clear();
            }
            continue;
        }

        // Парсим клю-значение
        if let Some((key, value)) = parse_key_value(trimmed) {
            current_record.insert(key.to_string(), value.to_string());
        }
    }

    // На случай если в конце файла нет пустой стр
    if !current_record.is_empty() {
        let operation = parse_record(&current_record)?;
        operation.validate()?;
        operations.insert(operation);
    }

    Ok(operations)
}

fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    line.split_once(':').map(|(k, v)| (k.trim(), v.trim()))
}

fn parse_record(record: &HashMap<String, String>) -> Result<Operation> {
    let tx_id = record
        .get("TX_ID")
        .ok_or_else(|| ParseError::InvalidFormat("Missing TX_ID".to_string()))?
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "TX_ID".to_string(),
            reason: e.to_string(),
        })?;

    let tx_type = OperationType::from_str(
        record
            .get("TX_TYPE")
            .ok_or_else(|| ParseError::InvalidFormat("Missing TX_TYPE".to_string()))?,
    )?;

    let from_user_id = record
        .get("FROM_USER_ID")
        .ok_or_else(|| ParseError::InvalidFormat("Missing FROM_USER_ID".to_string()))?
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "FROM_USER_ID".to_string(),
            reason: e.to_string(),
        })?;

    let to_user_id = record
        .get("TO_USER_ID")
        .ok_or_else(|| ParseError::InvalidFormat("Missing TO_USER_ID".to_string()))?
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "TO_USER_ID".to_string(),
            reason: e.to_string(),
        })?;

    let amount = record
        .get("AMOUNT")
        .ok_or_else(|| ParseError::InvalidFormat("Missing AMOUNT".to_string()))?
        .parse::<i64>()
        .map_err(|e| ParseError::InvalidField {
            field: "AMOUNT".to_string(),
            reason: e.to_string(),
        })?;

    let timestamp = record
        .get("TIMESTAMP")
        .ok_or_else(|| ParseError::InvalidFormat("Missing TIMESTAMP".to_string()))?
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "TIMESTAMP".to_string(),
            reason: e.to_string(),
        })?;

    let status = OperationStatus::from_str(
        record
            .get("STATUS")
            .ok_or_else(|| ParseError::InvalidFormat("Missing STATUS".to_string()))?,
    )?;

    let description = record
        .get("DESCRIPTION")
        .ok_or_else(|| ParseError::InvalidFormat("Missing DESCRIPTION".to_string()))?
        .trim_matches('"')
        .to_string();

    Ok(Operation {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

/// Записываем всё в txt
pub fn write_all<W: Write>(mut writer: W, operations: &HashSet<Operation>) -> Result<()> {
    for (i, operation) in operations.iter().enumerate() {
        operation.validate()?;

        if i > 0 {
            writeln!(writer)?;
        }

        writeln!(writer, "TX_ID: {}", operation.tx_id)?;
        writeln!(writer, "TX_TYPE: {}", operation.tx_type.as_str())?;
        writeln!(writer, "FROM_USER_ID: {}", operation.from_user_id)?;
        writeln!(writer, "TO_USER_ID: {}", operation.to_user_id)?;
        writeln!(writer, "AMOUNT: {}", operation.amount)?;
        writeln!(writer, "TIMESTAMP: {}", operation.timestamp)?;
        writeln!(writer, "STATUS: {}", operation.status.as_str())?;
        writeln!(writer, "DESCRIPTION: \"{}\"", operation.description)?;
    }

    Ok(())
}
