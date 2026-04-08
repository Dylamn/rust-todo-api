use crate::models::TaskDescriptionError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

// TODO: Implement the RFC 7807 (Problem Details)
#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound,

    #[error("{message}")]
    UnprocessableContent { code: &'static str, message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (http_status, code, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not_found", self.to_string()),

            ApiError::UnprocessableContent { code, message } => {
                (StatusCode::UNPROCESSABLE_ENTITY, code, message)
            }
        };

        let body = ErrorResponse {
            error: ErrorBody { code, message },
        };

        (http_status, Json(body)).into_response()
    }
}

impl From<TaskDescriptionError> for ApiError {
    fn from(error: TaskDescriptionError) -> Self {
        match error {
            TaskDescriptionError::Empty => Self::UnprocessableContent {
                code: "task_description_empty",
                message: error.to_string(),
            },
            TaskDescriptionError::TooLong => Self::UnprocessableContent {
                code: "task_description_too_long",
                message: error.to_string(),
            },
        }
    }
}
