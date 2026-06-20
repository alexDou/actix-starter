use serde::{Deserialize, Serialize};

use crate::domain::item::model::Item;
use crate::domain::user::model::UserResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub user: UserResponse,
    pub items: Vec<Item>,
}
