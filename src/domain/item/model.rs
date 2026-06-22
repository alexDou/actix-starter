use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::domain::user::model::UserResponse;
use crate::config::{RE_ITEM_NAME, RE_ITEM_DESCRIPTION};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsResponse {
    pub user: UserResponse,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ItemCreateRequestPayload {
    #[validate(
        length(min = 1, max = 64, message = "Please, keep the item name from 1 to 64 characters long"),
        regex(path = *RE_ITEM_NAME, message = "Please, keep the item name off fancy characters")
    )]
    pub name: String,
    #[validate(
        length(min = 16, max = 2048, message = "Please, keep the item description from 16 to 2048 characters long"),
        regex(path = *RE_ITEM_DESCRIPTION, message = "Please, use only typical characters in the item description")
    )]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemCreate<'ic> {
    pub user_id: &'ic str,
    pub item_payload: ItemCreateRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemResponse {
    pub item: Item,
}
