use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

use crate::domain::auth::lib::common::AuthenticatedUser;
use crate::domain::auth::lib::common::verify_request_by_params;
use crate::domain::item::{
    entity::{items_by_user, user_item_by_id},
    model::{ItemRequestParams, ItemResponse, ItemsResponse},
};
use crate::domain::user::entity::user_by_col_value;
use crate::domain::user::model::{
    UserDBQueryParameters, UserLookupField, UserRequestParams, UserResponse,
};
use crate::libs::errors::AppError;

pub async fn fetch_items(
    pool: web::Data<PgPool>,
    path_params: web::Path<UserRequestParams>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    verify_request_by_params(&user, &path_params.into_inner().uid)
        .map_err(|_| AppError::Unauthorized)
        .unwrap();

    let db_query_params = UserDBQueryParameters {
        col_name: UserLookupField::Id,
        value: user.user_id.clone(),
    };
    let user = user_by_col_value(&pool, &db_query_params).await?;
    let items = items_by_user(&pool, &user.id).await?;

    Ok(HttpResponse::Ok().json(ItemsResponse {
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

pub async fn fetch_item(
    pool: web::Data<PgPool>,
    path_params: web::Path<ItemRequestParams>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    let item_id = &path_params.into_inner().iid;
    let item = user_item_by_id(&pool, &item_id, &user.user_id).await?;

    Ok(HttpResponse::Ok().json(ItemResponse { item }))
}
