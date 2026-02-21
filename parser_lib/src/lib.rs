//! YPBank Operation Parser Library
//!
//! Либа для парсинга и сериализации операций:
//! - Binary format (YPBankBin)
//! - CSV format (YPBankCsv)
//! - Text format (YPBankText)
//!

pub mod bin_format;
pub mod csv_format;
pub mod error;
pub mod operation;
pub mod text_format;

pub use error::{ParseError, Result};
pub use operation::{Operation, OperationStatus, OperationType};

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, io::Cursor};

    fn create_test_operation() -> Operation {
        Operation {
            tx_id: 1234567890123456,
            tx_type: OperationType::Deposit,
            from_user_id: 0,
            to_user_id: 9876543210987654,
            amount: 10000,
            timestamp: 1633036800000,
            status: OperationStatus::Success,
            description: "Test deposit".to_string(),
        }
    }

    #[test]
    fn test_binary_round_trip() {
        let op = create_test_operation();
        let mut buf = Vec::new();

        bin_format::write_operation(&mut buf, &op).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = bin_format::parse_operation(&mut cursor).unwrap();

        assert_eq!(op, parsed);
    }

    #[test]
    fn test_csv_round_trip() {
        let operations: HashSet<Operation> = vec![create_test_operation()].into_iter().collect();
        let mut buf = Vec::new();

        csv_format::write_all(&mut buf, &operations).unwrap();

        let cursor = Cursor::new(buf);
        let parsed = csv_format::parse_all(cursor).unwrap();

        assert_eq!(operations, parsed);
    }

    #[test]
    fn test_text_round_trip() {
        let operations: HashSet<Operation> = vec![create_test_operation()].into_iter().collect();
        let mut buf = Vec::new();

        text_format::write_all(&mut buf, &operations).unwrap();

        let cursor = Cursor::new(buf);
        let parsed = text_format::parse_all(cursor).unwrap();

        assert_eq!(operations, parsed);
    }
}
