use std::time::Duration;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Postgres {
    #[serde(rename = "POSTGRES_HOST")]
    host: String,
    #[serde(rename = "POSTGRES_PORT", default = "default_postgres_port")]
    port: u16,
    #[serde(rename = "POSTGRES_DB")]
    db: String,
    #[serde(rename = "POSTGRES_USER")]
    user: String,
    #[serde(rename = "POSTGRES_PASSWORD")]
    password: String,
}

impl Postgres {
    fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "PORT", default = "default_port")]
    port: u16,

    #[serde(rename = "DOWNLOAD_DURATION")]
    download_duration_secs: u64,

    #[serde(rename = "IMAGE_URL")]
    image_url: String,

    #[serde(flatten)]
    postgres: Postgres,
}

impl Config {
    pub fn init() -> Result<Self> {
        // Try to load .env file, but don't fail if it doesn't exist (for Kubernetes)
        dotenvy::dotenv().ok();

        let config = match envy::from_env::<Config>() {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Config err: {}", e);
                std::process::exit(1);
            }
        };

        Ok(config)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn connection_string(&self) -> String {
        self.postgres.connection_string()
    }

    pub fn download_duration(&self) -> Duration {
        Duration::from_secs(self.download_duration_secs)
    }

    pub fn image_url(&self) -> String {
        self.image_url.clone()
    }
}

fn default_port() -> u16 {
    8080
}

fn default_postgres_port() -> u16 {
    5432
}
