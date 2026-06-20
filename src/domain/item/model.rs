use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::user::model::UserResponse;

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
