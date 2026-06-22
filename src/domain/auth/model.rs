use serde::{Deserialize, Serialize};
use validator::Validate;
use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

use crate::config::PASSWORD_SPECIAL_CHARS;

use crate::domain::auth::lib::validator::validate_password;
use crate::domain::user::model::UserResponse;

pub static RE_PASSWORD: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(r"^[a-z0-9_{}]{{8,128}}$", regex::escape(PASSWORD_SPECIAL_CHARS));
    RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .unicode(false)
        .build()
        .unwrap()
});

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 8, message = "Password must be between 8 and 128 characters long"),
        custom(function = "validate_password")
    )]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password confirmation failed"))]
    pub password_confirm: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(regex(path = *RE_PASSWORD))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}
