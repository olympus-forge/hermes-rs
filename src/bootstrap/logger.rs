use tracing_subscriber::{fmt, EnvFilter};

use crate::config::settings::Settings;

pub fn init(settings: &Settings) {
    let level = settings.log.log_level.clone();

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    let is_json = settings.log.log_format.to_lowercase() == "json";

    if is_json{
        fmt()
            .with_env_filter(filter)
            .json()
            .init();
        return;
    } else {
        fmt()
            .with_env_filter(filter)
            .pretty()
            .init();
    }
}