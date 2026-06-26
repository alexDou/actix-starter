use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::item::model::{Item, ItemCreate, ItemUpdate};
use crate::libs::errors::AppError;

pub async fn items_by_user(pool: PgPool, user_id: &Uuid) -> Result<Vec<Item>, AppError> {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE user_id = $1")
        .bind(&user_id)
        .fetch_all(&pool)
        .await?;

    Ok(items)
}

pub async fn user_item_by_id(pool: PgPool, item_id: &str, user_id: &str) -> Result<Item, AppError> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1 AND user_id = $2")
        .bind(&item_id)
        .bind(&user_id)
        .fetch_one(&pool)
        .await?;

    Ok(item)
}

pub async fn create_user_item(pool: PgPool, values: &ItemCreate<'_>) -> Result<Item, AppError> {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO items i (user_id, name, description) VALUES($1, $2, $3) RETURNING i.*",
    )
    .bind(&values.user_id)
    .bind(&values.item_payload.name)
    .bind(&values.item_payload.description)
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(item)
}

pub async fn update_user_item(pool: PgPool, values: &ItemUpdate<'_>) -> Result<Item, AppError> {
    let item = sqlx::query_as::<_, Item>(
        "UPDATE items i SET(name, description) VALUES($3, $4) WHERE user_id = $1 AND id = $2 RETURNING i.*",
    )
    .bind(&values.user_id)
    .bind(&values.item_id)
    .bind(&values.item_payload.name)
    .bind(&values.item_payload.description)
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(item)
}
