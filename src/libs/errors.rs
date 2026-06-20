use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;
use serde_json::json;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Internal server error")]
    InternalServerError,
    #[display("Bad request: {}", _0)]
    BadRequest(String),
    #[display("Unauthorized")]
    Unauthorized,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError()
                .json(json!({ "error": "Internal server execution fault" })),
            AppError::BadRequest(message) => {
                HttpResponse::BadRequest().json(json!({ "error": message }))
            }
            AppError::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "error": "Unauthorized pipeline request" })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::BadRequest("Identity attribute constraints already exist".to_string())
            }
            _ => AppError::InternalServerError,
        }
    }
}
