use config::{ConfigError, Environment, File};
use serde::Deserialize;

fn default_false() -> bool { false }

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub cg_url: String,
    pub cg_key: String,
    pub jwt_secret: String,
    #[serde(default = "default_false")]
    pub json_logs: bool,
    pub rust_log: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
