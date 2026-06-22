use std::env;
use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

use crate::libs::errors::AppError;

#[derive(Debug)]
pub struct DbConfig {
    pub host: String,
    pub port: String,
    pub db_name: String,
    pub user_name: String,
    pub user_pwd: String,
}

#[derive(Debug)]
pub struct APIConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub session_name: String,
    pub session_ttl_hrs: i64,
    pub cors_max_age: usize,
}

pub fn get_db_config() -> Result<DbConfig, AppError> {
    let db_config = DbConfig {
        host: match env::var("DATABASE_HOST") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
        port: match env::var("DATABASE_PORT") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
        db_name: match env::var("DATABASE_NAME") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
        user_name: match env::var("DATABASE_USER") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
        user_pwd: match env::var("DATABASE_USER_PWD") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
    };

    Ok(db_config)
}

pub fn get_api_config() -> Result<APIConfig, AppError> {
    let api_config = APIConfig {
        host: match env::var("SERVER_HOST") {
            Ok(val) => val,
            _ => String::from("localhost"),
        },
        port: match env::var("SERVER_PORT") {
            Ok(val) => val.parse::<u16>().unwrap(),
            _ => 80,
        },
        jwt_secret: match env::var("JWT_SECRET") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
        session_name: match env::var("SESSION_NAME") {
            Ok(val) => val,
            Err(_) => return Err(AppError::InternalServerError),
        },
        session_ttl_hrs: match env::var("SESSION_TTL_HRS") {
            Ok(val) => val
                .parse::<i64>()
                .map_err(|_| AppError::InternalServerError)?,
            _ => 18,
        },
        cors_max_age: match env::var("SESSION_TTL_HRS") {
            Ok(val) => val
                .parse::<usize>()
                .map_err(|_| AppError::InternalServerError)?,
            _ => 10,
        },
    };

    Ok(api_config)
}

pub const PASSWORD_SPECIAL_CHARS: &'static str = ",.-!?;:_@^*$%";
pub static RE_ITEM_NAME: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r"^[a-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u024F 0-9_:\-]+$";
    RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
});
pub static RE_ITEM_DESCRIPTION: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r#"^[a-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u024F 0-9_,\.!\?%&\$\(\)#:"\-]+$"#;
    RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
});