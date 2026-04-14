use crate::models::TaskDescriptionError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
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

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (http_status, code, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not_found", self.to_string()),

            ApiError::UnprocessableContent { code, message } => {
                (StatusCode::UNPROCESSABLE_ENTITY, code, message)
            }

            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                self.to_string(),
            ),
        };

        let body = ErrorResponse {
            error: ErrorBody { code, message },
        };

        (http_status, Json(body)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        match error.downcast::<TaskDescriptionError>() {
            Ok(e) => ApiError::from(e),
            Err(error) => {
                eprintln!("Error: {}", error);
                Self::InternalError
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn response_status_and_body(error: ApiError) -> (StatusCode, serde_json::Value) {
        let response = error.into_response();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        (status, body)
    }

    #[tokio::test]
    async fn not_found_returns_404_with_error_body() {
        let (status, body) = response_status_and_body(ApiError::NotFound).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["error"]["code"], "not_found");
        assert!(body["error"]["message"].is_string());
    }

    #[tokio::test]
    async fn unprocessable_content_returns_422_with_error_body() {
        let error = ApiError::UnprocessableContent {
            code: "task_description_empty",
            message: "The description cannot be empty".to_string(),
        };

        let (status, body) = response_status_and_body(error).await;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body["error"]["code"], "task_description_empty");
        assert_eq!(body["error"]["message"], "The description cannot be empty");
    }

    #[tokio::test]
    async fn from_empty_description_error_produces_correct_code() {
        let api_error = ApiError::from(TaskDescriptionError::Empty);

        let (status, body) = response_status_and_body(api_error).await;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body["error"]["code"], "task_description_empty");
    }

    #[tokio::test]
    async fn from_too_long_description_error_produces_correct_code() {
        let api_error = ApiError::from(TaskDescriptionError::TooLong);

        let (status, body) = response_status_and_body(api_error).await;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body["error"]["code"], "task_description_too_long");
    }
}
