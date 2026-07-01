use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Display, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserLookupValue {
    String(String),
    Uuid(Uuid),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserLookupField {
    Id,
    Email,
    Username,
}

impl UserLookupField {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserLookupField::Id => "id",
            UserLookupField::Email => "email",
            UserLookupField::Username => "username",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDBQueryParameters {
    pub col_name: UserLookupField,
    pub value: UserLookupValue,
}
impl UserDBQueryParameters {
    pub fn by_id(id: Uuid) -> Self {
        Self { col_name: UserLookupField::Id, value: UserLookupValue::Uuid(id) }
    }
    pub fn by_email(email: String) -> Self {
        Self { col_name: UserLookupField::Email, value: UserLookupValue::String(email) }
    }
    pub fn by_username(username: String) -> Self {
        Self { col_name: UserLookupField::Username, value: UserLookupValue::String(username) }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequestParams {
    pub uid: String,
}
