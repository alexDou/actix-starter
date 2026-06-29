use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;

use crate::config::AppData;
use crate::domain::auth::{lib::common::hash_password, model::RegisterRequest};
use crate::domain::user::{entity::create_user, model::UserResponse};
use crate::libs::errors::AppError;

pub async fn create_register(
    app_data: web::Data<AppData>,
    body: Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let payload = body.into_inner();
    
    let hash = hash_password(&payload.password).map_err(|_| AppError::InternalServerError)?;
    let user = create_user(app_data.pg_pool.clone(), &payload.email, &hash).await?;

    Ok(HttpResponse::Created().json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}
