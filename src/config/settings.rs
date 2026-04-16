use anyhow::Result;
use config::{Config, File, Environment};
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub app_version: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub log_level: String,
    pub log_format: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
   pub provider: String,
   pub from_email: String,
   pub from_name: String,
   pub smtp: SmtpConfig,
}


#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub app: AppConfig,
    pub log: LogConfig,
    pub email: EmailConfig,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let cfg = Config::builder()
           // Load configs/default.toml
            .add_source(File::with_name("configs/default").required(true))
            .add_source(
                Environment::default()
                .separator("_")
                .try_parsing(true),
            )
            .build()?;

        let settings: Settings = cfg.try_deserialize()?;
        Ok(settings)
    }
}