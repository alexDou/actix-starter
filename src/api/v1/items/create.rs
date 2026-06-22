use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
use sqlx::PgPool;

use crate::domain::auth::lib::common::AuthenticatedUser;
use crate::domain::item::{
    entity::create_user_item,
    model::{ItemCreate, ItemCreateRequestPayload, ItemResponse},
};
use crate::libs::errors::AppError;

pub async fn create_item(
    pool: web::Data<PgPool>,
    body: Json<ItemCreateRequestPayload>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    let params = ItemCreate {
        user_id: &user.user_id,
        item_payload: ItemCreateRequestPayload {
            name: body.name.to_owned(),
            description: body.description.to_owned(),
        },
    };
    let item = create_user_item(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(ItemResponse { item }))
}
