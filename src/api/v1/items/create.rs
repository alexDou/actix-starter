use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;

use crate::config::AppData;
use crate::domain::auth::lib::common::AuthenticatedUser;
use crate::domain::item::{
    entity::create_user_item,
    model::{ItemCreate, ItemRequestPayload, ItemResponse},
};
use crate::libs::errors::AppError;

pub async fn create_item(
    app_data: web::Data<AppData>,
    body: Json<ItemRequestPayload>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    let params = ItemCreate {
        user_id: &user.user_id,
        item_payload: ItemRequestPayload {
            name: body.name.to_owned(),
            description: body.description.to_owned(),
        },
    };
    let item = create_user_item(app_data.pg_pool.clone(), &params).await?;

    Ok(HttpResponse::Ok().json(ItemResponse { item }))
}
