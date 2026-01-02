use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub database_url: String,
    pub environment: String,
}

impl AppConfig {
    pub fn from_env_and_file() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();
        let settings = config::Config::builder()
            .add_source(config::File::with_name("reference"))
            .add_source(config::Environment::default().separator("_"))
            .build()?;
        settings.try_deserialize::<AppConfig>()
    }
}
