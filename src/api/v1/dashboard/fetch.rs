use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

use crate::domain::auth::lib::common::AuthenticatedUser;
use crate::domain::dashboard::model::DashboardResponse;
use crate::domain::user::entity::user_by_col_value;
use crate::domain::item::entity::items_by_user;
use crate::domain::user::{model::{UserQueryParameters, UserLookupField, UserResponse}};
use crate::libs::errors::AppError;

pub async fn fetch_user_items(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    let params = UserQueryParameters {
        col_name: UserLookupField::Id,
        value: &user.user_id,
    };
    let user = user_by_col_value(&pool, &params).await?;
    let items = items_by_user(&pool, &user.id).await?;

    Ok(HttpResponse::Ok().json(DashboardResponse {
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        },
        items,
    }))
}
