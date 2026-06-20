use actix_web::{HttpResponse, Responder, web};
use actix_session::Session;
use sqlx::PgPool;

use crate::domain::auth::{lib::common::verify_password, lib::keys::init_session, model::RegisterRequest};
use crate::domain::user::entity::user_by_col_value;
use crate::domain::user::{model::{UserQueryParameters, UserLookupField, UserResponse}};
use crate::libs::errors::AppError;

pub async fn create_session(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterRequest>,
    session: Session,
) -> Result<impl Responder, AppError> {
    let params = UserQueryParameters {
        col_name: UserLookupField::Username,
        value: &body.email,
    };
    let user = user_by_col_value(&pool, &params).await?;

    match verify_password(&body.password, &user.password_hash) {
        Ok(_) => {
            init_session(&session, &user.id)?;

            Ok(HttpResponse::Ok().json(UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }))
        },
        Err(_) => Err(AppError::Unauthorized)
    }
}
