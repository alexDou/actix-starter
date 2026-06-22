use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::item::model::{Item, ItemCreate};
use crate::libs::errors::AppError;

pub async fn items_by_user(
    pool: &web::Data<PgPool>,
    user_id: &Uuid,
) -> Result<Vec<Item>, AppError> {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE user_id = #1")
        .bind(&user_id)
        .fetch_all(pool.get_ref())
        .await?;

    Ok(items)
}

pub async fn create_user_item(
    pool: &web::Data<PgPool>,
    values: &ItemCreate<'_>,
) -> Result<Item, AppError> {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO items i (user_id, name, description) VALUES($1, $2, $3) RETURNING i.*",
    )
    .bind(&values.user_id)
    .bind(&values.item_payload.name)
    .bind(&values.item_payload.description)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(item)
}
