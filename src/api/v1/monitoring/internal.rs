use actix_web::{HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::libs::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct HealhCheckRespond {
    status: String,
}

pub async fn health_check() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().json(HealhCheckRespond {
        status: String::from("health"),
    }))
}
