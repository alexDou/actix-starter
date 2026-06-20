use crate::libs::errors::AppError;
use actix_session::Session;
use uuid::Uuid;

use crate::domain::auth::lib::common::create_jwt;

pub fn init_session(session: &Session, user_id: &Uuid) -> Result<String, AppError> {
    let token = create_jwt(&user_id).map_err(|_| AppError::InternalServerError)?;
    session
        .insert("user_id", &user_id)
        .map_err(|_| AppError::InternalServerError)?;

    Ok(token)
}
