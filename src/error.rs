use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Invalid ID format")]
    InvalidId,

    #[error("Database error")]
    DatabaseError(#[from] mongodb::error::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidId => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let code = match self {
            AppError::NotFound => "NOT_FOUND",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::InvalidId => "INVALID_ID",
            AppError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                "DATABASE_ERROR"
            }
        };

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            code,
            message: self.to_string(),
        })
    }
}
