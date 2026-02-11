use crate::error::{ParseError, Result};
use crate::operation::{Operation, OperationStatus, OperationType};
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

/// Нофинг интерестинг, ходим по строкам, парсим
pub fn parse_all<R: Read>(reader: R) -> Result<HashSet<Operation>> {
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let header = lines.next().ok_or(ParseError::UnexpectedEof)??;

    if header != HEADER {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid CSV header. Expected: {}",
            HEADER
        )));
    }

    let mut operations = HashSet::new();

    for (line_num, line) in lines.enumerate() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let operation: Operation = parse_line(&line)
            .map_err(|e| ParseError::InvalidFormat(format!("Line {}: {}", line_num + 2, e)))?;

        operation.validate()?;
        operations.insert(operation);
    }

    Ok(operations)
}

fn parse_line(line: &str) -> Result<Operation> {
    let parts: Vec<&str> = split_csv_line(line);

    if parts.len() != 8 {
        return Err(ParseError::InvalidFormat(format!(
            "Expected 8 fields, got {}",
            parts.len()
        )));
    }

    let tx_id = parts[0]
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "TX_ID".to_string(),
            reason: e.to_string(),
        })?;

    let tx_type = OperationType::from_str(parts[1])?;

    let from_user_id = parts[2]
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "FROM_USER_ID".to_string(),
            reason: e.to_string(),
        })?;

    let to_user_id = parts[3]
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "TO_USER_ID".to_string(),
            reason: e.to_string(),
        })?;

    let amount = parts[4]
        .parse::<i64>()
        .map_err(|e| ParseError::InvalidField {
            field: "AMOUNT".to_string(),
            reason: e.to_string(),
        })?;

    let timestamp = parts[5]
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidField {
            field: "TIMESTAMP".to_string(),
            reason: e.to_string(),
        })?;

    let status = OperationStatus::from_str(parts[6])?;

    let description = parts[7].trim_matches('"').to_string();

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

fn split_csv_line(line: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;

    for (i, c) in line.char_indices() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            parts.push(&line[start..i]);
            start = i + 1;
        }
    }
    parts.push(&line[start..]);

    parts
}

/// Пишем всё в csv
pub fn write_all<W: Write>(mut writer: W, operations: &HashSet<Operation>) -> Result<()> {
    writeln!(writer, "{}", HEADER)?;

    for operation in operations {
        operation.validate()?;

        writeln!(
            writer,
            "{},{},{},{},{},{},{},\"{}\"",
            operation.tx_id,
            operation.tx_type.as_str(),
            operation.from_user_id,
            operation.to_user_id,
            operation.amount,
            operation.timestamp,
            operation.status.as_str(),
            operation.description
        )?;
    }

    Ok(())
}
