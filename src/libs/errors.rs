use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;
use serde_json::json;
use validator::ValidationError;

#[derive(Debug, Display)]
pub enum AppError {
    InternalServerError,
    BadRequest(String),
    Unauthorized,
    RefferenceError(String),
    DataValidationError(ValidationError),
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::DataValidationError(err)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError()
                .json(json!({ "error": "Internal server error" })),
            AppError::BadRequest(message) => {
                HttpResponse::BadRequest().json(json!({ "error": message }))
            }
            AppError::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "error": "Unauthorized pipeline request" })),
            AppError::RefferenceError(message) => {
                HttpResponse::InternalServerError().json(json!({ "error": message }))
            },
            AppError::DataValidationError(err) => {
                HttpResponse::BadRequest().json(json!({ "error": err.to_owned().message }))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::RefferenceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DataValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::BadRequest("Identity attribute constraints already exist".to_owned())
            }
            _ => AppError::InternalServerError,
        }
    }
}
