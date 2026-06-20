use actix_web::web;
use sqlx::PgPool;

use ::uuid::Uuid;

use crate::domain::item::model::Item;
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
