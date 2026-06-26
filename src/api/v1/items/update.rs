use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;

use crate::config::AppData;
use crate::domain::auth::lib::common::AuthenticatedUser;
use crate::domain::item::{
    entity::update_user_item,
    model::{ItemUpdate, ItemRequestPayload, ItemRequestParams, ItemResponse},
};
use crate::libs::errors::AppError;

pub async fn update_item(
    app_data: web::Data<AppData>,
    path_params: web::Path<ItemRequestParams>,
    body: Json<ItemRequestPayload>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    let params = ItemUpdate {
        item_id: &path_params.into_inner().iid,
        user_id: &user.user_id,
        item_payload: ItemRequestPayload {
            name: body.name.to_owned(),
            description: body.description.to_owned(),
        },
    };
    let item = update_user_item(app_data.pg_pool.clone(), &params).await?;

    Ok(HttpResponse::Ok().json(ItemResponse { item }))
}
