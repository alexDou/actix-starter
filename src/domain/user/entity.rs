use actix_web::web;
use sqlx::{PgPool, QueryBuilder};

use crate::domain::user::model::{User, UserQueryParameters};
use crate::libs::errors::AppError;

pub async fn user_by_col_value(
    pool: &web::Data<PgPool>,
    params: &UserQueryParameters,
) -> Result<User, AppError> {
    QueryBuilder::new(format!(
        "SELECT * FROM users WHERE {:?} = {}",
        &params.col_name,
        params.value
    ))
    .build_query_as::<User>()
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::BadRequest(String::from("Invalid application login credentials")))
}

pub async fn create_user(
    pool: &web::Data<PgPool>,
    email: &str,
    password_hash: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, username, password_hash, created_at, updated_at"
    )
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(user)
}
