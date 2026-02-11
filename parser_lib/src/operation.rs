use crate::error::{ParseError, Result};
use std::hash::Hash;

/// Тип финансовой операции
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    /// Пополнение счета
    Deposit,
    /// Перевод между пользователями
    Transfer,
    /// Снятие средств
    Withdrawal,
}

impl OperationType {
    /// Парсит тип операции из строки
    ///
    /// # Аргументы
    /// * `s` - Строковое представление типа операции ("DEPOSIT", "TRANSFER", "WITHDRAWAL")
    ///
    /// # Возвращает
    /// * `Ok(OperationType)` - Если строка корректна
    /// * `Err(ParseError)` - Если строка не распознана
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

    /// Создает тип операции из числового значения
    ///
    /// # Аргументы
    /// * `value` - Числовое представление (0 = Deposit, 1 = Transfer, 2 = Withdrawal)
    ///
    /// # Возвращает
    /// * `Ok(OperationType)` - Если значение корректно
    /// * `Err(ParseError)` - Если значение неизвестно
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

    /// Конвертирует тип операции в числовое значение
    ///
    /// # Возвращает
    /// * `0` для Deposit
    /// * `1` для Transfer
    /// * `2` для Withdrawal
    pub fn to_u8(&self) -> u8 {
        match self {
            OperationType::Deposit => 0,
            OperationType::Transfer => 1,
            OperationType::Withdrawal => 2,
        }
    }

    /// Возвращает строковое представление типа операции
    ///
    /// # Возвращает
    /// Строку "DEPOSIT", "TRANSFER" или "WITHDRAWAL"
    pub fn as_str(&self) -> &str {
        match self {
            OperationType::Deposit => "DEPOSIT",
            OperationType::Transfer => "TRANSFER",
            OperationType::Withdrawal => "WITHDRAWAL",
        }
    }
}

/// Статус выполнения операции
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationStatus {
    /// Операция успешно выполнена
    Success,
    /// Операция завершилась с ошибкой
    Failure,
    /// Операция в процессе выполнения
    Pending,
}

impl OperationStatus {
    /// Парсит статус операции из строки
    ///
    /// # Аргументы
    /// * `s` - Строковое представление статуса ("SUCCESS", "FAILURE", "PENDING")
    ///
    /// # Возвращает
    /// * `Ok(OperationStatus)` - Если строка корректна
    /// * `Err(ParseError)` - Если строка не распознана
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

    /// Создает статус операции из числового значения
    ///
    /// # Аргументы
    /// * `value` - Числовое представление (0 = Success, 1 = Failure, 2 = Pending)
    ///
    /// # Возвращает
    /// * `Ok(OperationStatus)` - Если значение корректно
    /// * `Err(ParseError)` - Если значение неизвестно
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

    /// Конвертирует статус операции в числовое значение
    ///
    /// # Возвращает
    /// * `0` для Success
    /// * `1` для Failure
    /// * `2` для Pending
    pub fn to_u8(&self) -> u8 {
        match self {
            OperationStatus::Success => 0,
            OperationStatus::Failure => 1,
            OperationStatus::Pending => 2,
        }
    }

    /// Возвращает строковое представление статуса операции
    ///
    /// # Возвращает
    /// Строку "SUCCESS", "FAILURE" или "PENDING"
    pub fn as_str(&self) -> &str {
        match self {
            OperationStatus::Success => "SUCCESS",
            OperationStatus::Failure => "FAILURE",
            OperationStatus::Pending => "PENDING",
        }
    }
}

/// Структура, представляющая финансовую операцию
#[derive(Debug, Clone, Eq)]
pub struct Operation {
    /// Уникальный идентификатор транзакции
    pub tx_id: u64,
    /// Тип операции (пополнение, перевод, снятие)
    pub tx_type: OperationType,
    /// ID пользователя-отправителя (0 для пополнений)
    pub from_user_id: u64,
    /// ID пользователя-получателя (0 для снятий)
    pub to_user_id: u64,
    /// Сумма операции
    pub amount: i64,
    /// Unix timestamp операции
    pub timestamp: u64,
    /// Статус выполнения операции
    pub status: OperationStatus,
    /// Описание операции
    pub description: String,
}

impl Operation {
    /// Валидирует корректность полей операции в зависимости от её типа
    ///
    /// # Правила валидации
    /// * **DEPOSIT**: `from_user_id` должен быть равен 0
    /// * **WITHDRAWAL**: `to_user_id` должен быть равен 0
    /// * **TRANSFER**: `from_user_id` и `to_user_id` не должны быть равны 0
    ///
    /// # Возвращает
    /// * `Ok(())` - Если операция валидна
    /// * `Err(ParseError)` - Если обнаружены некорректные поля
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
