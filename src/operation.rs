use crate::error::{ParseError, Result};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Deposit,
    Transfer,
    Withdrawal,
}

impl OperationType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "DEPOSIT" => Ok(OperationType::Deposit),
            "TRANSFER" => Ok(OperationType::Transfer),
            "WITHDRAWAL" => Ok(OperationType::Withdrawal),
            _ => Err(ParseError::InvalidField {
                field: "TX_TYPE".to_string(),
                reason: format!("Unknown transaction type: {}", s),
            }),
        }
    }

    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(OperationType::Deposit),
            1 => Ok(OperationType::Transfer),
            2 => Ok(OperationType::Withdrawal),
            _ => Err(ParseError::InvalidField {
                field: "TX_TYPE".to_string(),
                reason: format!("Unknown transaction type value: {}", value),
            }),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            OperationType::Deposit => 0,
            OperationType::Transfer => 1,
            OperationType::Withdrawal => 2,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            OperationType::Deposit => "DEPOSIT",
            OperationType::Transfer => "TRANSFER",
            OperationType::Withdrawal => "WITHDRAWAL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationStatus {
    Success,
    Failure,
    Pending,
}

impl OperationStatus {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "SUCCESS" => Ok(OperationStatus::Success),
            "FAILURE" => Ok(OperationStatus::Failure),
            "PENDING" => Ok(OperationStatus::Pending),
            _ => Err(ParseError::InvalidField {
                field: "STATUS".to_string(),
                reason: format!("Unknown status: {}", s),
            }),
        }
    }

    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(OperationStatus::Success),
            1 => Ok(OperationStatus::Failure),
            2 => Ok(OperationStatus::Pending),
            _ => Err(ParseError::InvalidField {
                field: "STATUS".to_string(),
                reason: format!("Unknown status value: {}", value),
            }),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            OperationStatus::Success => 0,
            OperationStatus::Failure => 1,
            OperationStatus::Pending => 2,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            OperationStatus::Success => "SUCCESS",
            OperationStatus::Failure => "FAILURE",
            OperationStatus::Pending => "PENDING",
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Operation {
    pub tx_id: u64,
    pub tx_type: OperationType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: OperationStatus,
    pub description: String,
}

impl Operation {
    pub fn validate(&self) -> Result<()> {
        match self.tx_type {
            OperationType::Deposit => {
                if self.from_user_id != 0 {
                    return Err(ParseError::InvalidField {
                        field: "FROM_USER_ID".to_string(),
                        reason: "Must be 0 for DEPOSIT".to_string(),
                    });
                }
            }
            OperationType::Withdrawal => {
                if self.to_user_id != 0 {
                    return Err(ParseError::InvalidField {
                        field: "TO_USER_ID".to_string(),
                        reason: "Must be 0 for WITHDRAWAL".to_string(),
                    });
                }
            }
            OperationType::Transfer => {
                if self.from_user_id == 0 || self.to_user_id == 0 {
                    return Err(ParseError::InvalidField {
                        field: "FROM_USER_ID/TO_USER_ID".to_string(),
                        reason: "Cannot be 0 for TRANSFER".to_string(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl Hash for Operation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tx_id.hash(state);
    }
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        self.tx_id == other.tx_id
    }
}
