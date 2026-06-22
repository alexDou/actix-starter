use regex::{Regex, RegexBuilder};
use std::env;
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

#[derive(Debug)]
pub struct AppConfig {
    pub db: DbConfig,
    pub api: APIConfig,
}

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    AppConfig::load().expect("Fatal error: Failed to parse runtime application configuration")
});

impl AppConfig {
    fn load() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            db: Self::load_db()?,
            api: Self::load_api()?,
        })
    }

    fn load_db() -> Result<DbConfig, AppError> {
        Ok(DbConfig {
            host: env::var("DATABASE_HOST").map_err(|_| {
                AppError::RefferenceError(String::from("env::DATABASE_HOST is not set"))
            })?,
            port: env::var("DATABASE_PORT").map_err(|_| {
                AppError::RefferenceError(String::from("env::DATABASE_PORT is not set"))
            })?,
            db_name: env::var("DATABASE_NAM").map_err(|_| {
                AppError::RefferenceError(String::from("env::DATABASE_NAME is not set"))
            })?,
            user_name: env::var("DATABASE_USER").map_err(|_| {
                AppError::RefferenceError(String::from("env::DATABASE_USER is not set"))
            })?,
            user_pwd: env::var("DATABASE_USER_PWD").map_err(|_| {
                AppError::RefferenceError(String::from("env::DATABASE_USER_PWD is not set"))
            })?,
        })
    }

    fn load_api() -> Result<APIConfig, AppError> {
        Ok(APIConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| String::from("localhost")),

            port: match env::var("SERVER_PORT") {
                Ok(val) => val.parse::<u16>().map_err(|_| {
                    AppError::RefferenceError(String::from("env::SERVER_HOST is not set"))
                })?,
                _ => 80,
            },

            jwt_secret: env::var("JWT_SECRET").map_err(|_| {
                AppError::RefferenceError(String::from("env::JWT_SECRET is not set"))
            })?,
            session_name: env::var("SESSION_NAME").map_err(|_| {
                AppError::RefferenceError(String::from("env::SESSION_NAME is not set"))
            })?,

            session_ttl_hrs: match env::var("SESSION_TTL_HRS") {
                Ok(val) => val.parse::<i64>().map_err(|_| {
                    AppError::RefferenceError(String::from("env::SESSION_TTL_HRS is not set"))
                })?,
                _ => 18,
            },

            cors_max_age: match env::var("CORS_MAX_AGE") {
                Ok(val) => val.parse::<usize>().map_err(|_| {
                    AppError::RefferenceError(String::from("env::CORS_MAX_AGE is not set"))
                })?,
                _ => 10,
            },
        })
    }
}

pub const PASSWORD_SPECIAL_CHARS: &str = ",.-!?;:_@^*$%";

pub static RE_ITEM_NAME: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r"^[a-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u024F 0-9_:\-]+$";
    RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
});

pub static RE_ITEM_DESCRIPTION: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r#"^[a-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u024F 0-9_,\.!\?%&\$\(\)#:"\-]+$"#;
    RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
});
