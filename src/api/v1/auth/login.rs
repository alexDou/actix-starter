use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
use sqlx::PgPool;

use crate::domain::auth::{
    lib::{common::verify_password, keys::init_session},
    model::LoginRequest,
};
use crate::domain::user::{
    entity::user_by_col_value,
    model::{UserLookupField, UserDBQueryParameters, UserResponse},
};
use crate::libs::errors::AppError;

pub async fn create_session(
    pool: web::Data<PgPool>,
    body: Json<LoginRequest>,
    session: Session,
) -> Result<impl Responder, AppError> {
    let payload = body.into_inner();
    let params = UserDBQueryParameters {
        col_name: UserLookupField::Username,
        value: payload.email,
    };
    let user = user_by_col_value(&pool, &params).await?;

    match verify_password(&payload.password, &user.password_hash) {
        Ok(_) => {
            init_session(&session, &user.id)?;

            Ok(HttpResponse::Ok().json(UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }))
        }
        Err(_) => Err(AppError::Unauthorized),
    }
}
