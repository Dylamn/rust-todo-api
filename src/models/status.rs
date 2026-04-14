use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, sqlx::Decode, sqlx::Encode)]
pub enum Status {
    Pending,
    Completed,
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "pending" => Status::Pending,
            "completed" => Status::Completed,
            _ => Status::Pending,
        }
    }
}

impl sqlx::Type<sqlx::Sqlite> for Status {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}
