use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskDescriptionError {
    #[error("The description cannot be empty")]
    Empty,
    #[error("The description cannot be longer than 100 bytes")]
    TooLong,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Decode, sqlx::Encode)]
#[serde(try_from = "String")]
pub struct TaskDescription(String);

impl TryFrom<String> for TaskDescription {
    type Error = TaskDescriptionError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl AsRef<str> for TaskDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for TaskDescription {
    type Error = TaskDescriptionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(TaskDescriptionError::Empty);
        }
        if value.len() > 100 {
            return Err(TaskDescriptionError::TooLong);
        }

        Ok(Self(value.to_string()))
    }
}

impl sqlx::Type<sqlx::Sqlite> for TaskDescription {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<Sqlite>>::type_info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_from_string() {
        let input = "hello".to_string();
        let desc = TaskDescription::try_from(input).unwrap();
        assert_eq!(desc.as_ref(), "hello");
    }

    #[test]
    fn should_create_description_with_exact_limit() {
        let desc = "a".repeat(100);
        assert!(TaskDescription::try_from(desc).is_ok())
    }

    #[test]
    fn should_return_empty_error() {
        let err = TaskDescription::try_from("").unwrap_err();
        assert!(matches!(err, TaskDescriptionError::Empty));
    }

    #[test]
    fn should_return_too_long_error() {
        let long = "a".repeat(101);
        let err = TaskDescription::try_from(long).unwrap_err();
        assert!(matches!(err, TaskDescriptionError::TooLong));
    }

    #[test]
    fn should_reject_whitespace_only() {
        assert!(TaskDescription::try_from("   ").is_err());
    }

    #[test]
    fn should_trim_description() {
        let desc = TaskDescription::try_from("  hello  ").unwrap();
        assert_eq!(desc.as_ref(), "hello");
    }

    #[test]
    fn should_handle_multibyte_characters() {
        let desc = "é".repeat(100); // 2 bytes each → 200 bytes
        assert!(TaskDescription::try_from(desc).is_err());
    }

    #[test]
    fn should_deserialize_valid_description() {
        let json = "\"valid description\"";
        let desc: TaskDescription = serde_json::from_str(json).unwrap();
        assert_eq!(desc.as_ref(), "valid description");
    }

    #[test]
    fn should_fail_deserialize_invalid_description() {
        let json = "\"\"";
        let result: Result<TaskDescription, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
