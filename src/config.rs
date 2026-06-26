use actix_web::{http::header::AUTHORIZATION, web};
use prometheus_client::registry::Registry;
use prometheus_client::{
    encoding::{EncodeLabelSet, text::encode},
    metrics::{family::Family, gauge::Gauge},
    registry::Registry,
};
use regex::{Regex, RegexBuilder};
use sqlx::PgPool;
use std::{env, sync::LazyLock, time::Duration};

use crate::libs::errors::AppError;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct DependencyLabels {
    pub dependency: String,
}

/// Centralized state for the Prometheus Registry and fast-access Metric Families
pub struct AppMetrics {
    pub registry: Registry,
    pub dependency_health: Family<DependencyLabels, Gauge>,
}

impl AppMetrics {
    pub fn new() -> Self {
        let mut registry = Registry::default();
        let dependency_health = Family::<DependencyLabels, Gauge>::default();

        registry.register(
            "app_dependency_healthy",
            "Indicates if an external dependency is healthy (1) or down (0)",
            dependency_health.clone(),
        );

        Self {
            registry,
            dependency_health,
        }
    }
}

#[derive(Debug)]
pub struct DbConfig {
    pub host: String,
    pub port: String,
    pub db_name: String,
    pub user_name: String,
    pub user_pwd: String,
}

#[derive(Debug)]
pub struct RedisCacheConfig {
    pub host: String,
    pub port: String,
    pub ttl: u64,
    pub key_prefix: String,
}

#[derive(Debug)]
pub struct APIConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub session_name: String,
    pub session_ttl_hrs: i64,
    pub cors_max_age: usize,
    pub metrics_token: String,
    pub metrics_ttl: u64,
}

#[derive(Debug)]
pub struct AppConfig {
    pub db: DbConfig,
    pub cache: RedisCacheConfig,
    pub api: APIConfig,
}

#[derive(Debug, Clone)]
pub struct AppData {
    pub pg_pool: PgPool,
    pub metrics: AppMetrics,
}

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    AppConfig::load().expect("Fatal error: Failed to parse runtime application configuration")
});

impl AppConfig {
    fn load() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            db: Self::load_db()?,
            cache: Self::load_cache()?,
            api: Self::load_api()?,
        })
    }

    fn load_cache() -> Result<RedisCacheConfig, AppError> {
        Ok(RedisCacheConfig {
            host: env::var("REDIS_HOST").map_err(|_| {
                AppError::RefferenceError(String::from("env::Redis_HOST is not set"))
            })?,
            port: env::var("REDIS_PORT").map_err(|_| {
                AppError::RefferenceError(String::from("env::REDIS_PORT is not set"))
            })?,
            ttl: match env::var("REDIS_TTL") {
                Ok(val) => val.parse::<u64>().map_err(|_| {
                    AppError::RefferenceError(String::from("env::REDIS_TTL is not set"))
                })?,
                _ => 60,
            },
            key_prefix: env::var("REDIS_KEY_PREFIX").map_err(|_| {
                AppError::RefferenceError(String::from("env::REDIS_KET_PREFIX is not set"))
            })?,
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
            db_name: env::var("DATABASE_NAME").map_err(|_| {
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
            metrics_token: env::var("METRICS_TOKEN").map_err(|_| {
                AppError::RefferenceError(String::from("env::METRICS_TOKEN is not set"))
            })?,
            metrics_ttl: match env::var("METRICS_TTL_MS") {
                Ok(val) => val.parse::<u64>().map_err(|_| {
                    AppError::RefferenceError(String::from("env::METRICS_TTL_MS is not set"))
                })?,
                _ => 18,
            },
        })
    }
}

pub static PASSWORD_SPECIAL_CHARS: &str = ",.-!?;:_@^*$%";

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
