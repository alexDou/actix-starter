use env_logger::builder;
use log::LevelFilter;

use crate::config::{APP_CONFIG, EnvironmentEnum};

pub fn env_logger_init() {
    let log_level = match APP_CONFIG.environment.environ {
        EnvironmentEnum::DEVELOPMENT => LevelFilter::Debug,
        EnvironmentEnum::STAGING => LevelFilter::Warn,
        EnvironmentEnum::PRODUCTION => LevelFilter::Info,
    };
    builder().filter_level(log_level).init();
}
