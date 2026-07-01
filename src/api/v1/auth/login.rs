use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;

use crate::config::AppData;
use crate::domain::auth::{
    lib::{common::verify_password, keys::init_session},
    model::LoginRequest,
};
use crate::domain::user::{
    entity::user_by_col_value,
    model::{UserDBQueryParameters, LoginResponse},
};
use crate::libs::errors::AppError;

pub async fn create_session(
    app_data: web::Data<AppData>,
    body: Json<LoginRequest>,
    session: Session,
) -> Result<impl Responder, AppError> {
    let payload = body.into_inner();
    let params = UserDBQueryParameters::by_email(payload.email);

    let user = user_by_col_value(app_data.pg_pool.clone(), &params).await?;

    match verify_password(&payload.password, &user.password_hash) {
        Ok(_) => {
            // re-generate on each successful login
            let token = init_session(&session, &user.id)?;

            Ok(HttpResponse::Ok().json(LoginResponse {
                token,
            }))
        }
        Err(_) => Err(AppError::Unauthorized),
    }
}
