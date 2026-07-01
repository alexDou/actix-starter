use crate::libs::errors::AppError;
use actix_session::{Session, SessionStatus};
use uuid::Uuid;

use crate::domain::auth::lib::common::create_jwt;

pub fn is_active_session(session: &Session) -> bool {
    match session.status() {
        | SessionStatus::Renewed
        | SessionStatus::Changed
        | SessionStatus::Unchanged => true,
        _ => false,
    }
}

pub fn init_session(session: &Session, user_id: &Uuid) -> Result<String, AppError> {
    let token = create_jwt(&user_id).map_err(|_| AppError::InternalServerError)?;
    session
        .insert("user_id", &user_id)
        .map_err(|_| AppError::InternalServerError)?;

    Ok(token)
}
