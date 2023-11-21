use dotenv::dotenv;
use eyre::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    pub port: u16,
    pub db_url: String,
    pub api_user: String,
    pub api_password: String,
}

impl EnvConfig {
    pub fn get() -> Result<EnvConfig> {
        dotenv().ok();

        let config = envy::prefixed("APP_").from_env::<EnvConfig>()?;

        Ok(config)
    }
}
