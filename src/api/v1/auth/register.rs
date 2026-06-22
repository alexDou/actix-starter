use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
use sqlx::PgPool;

use crate::domain::auth::{lib::common::hash_password, model::RegisterRequest};
use crate::domain::user::{entity::create_user, model::UserResponse};
use crate::libs::errors::AppError;

pub async fn create_register(
    pool: web::Data<PgPool>,
    body: Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let payload = body.into_inner();
    let hash = hash_password(&payload.password).map_err(|_| AppError::InternalServerError)?;
    let user = create_user(&pool, &payload.email, &hash).await?;

    Ok(HttpResponse::Created().json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}
