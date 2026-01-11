use std::time::Duration;
use anyhow::Result;
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
struct Postgres {
    host: String,
    port: u16,
    db: String,
    user: String,
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
    port: u16,

    download_duration_secs: u64,

    image_url: String,

    postgres: Postgres,
}

impl Config {
    #[instrument]
    pub fn init() -> Result<Self> {
        dotenvy::dotenv().ok();

        let port: u16 = std::env::var("PORT")?.parse()?;
        let image_url = std::env::var("IMAGE_URL")?;
        let download_duration: u64 = std::env::var("DOWNLOAD_DURATION")?.parse()?;

        let postgres_host = std::env::var("POSTGRES_HOST")?;
        let postgres_port: u16 = std::env::var("POSTGRES_PORT")?.parse()?;
        let postgres_db = std::env::var("POSTGRES_DB")?;
        let postgres_user = std::env::var("POSTGRES_USER")?;
        let postgres_password = std::env::var("POSTGRES_PASSWORD")?;

        let config = Config {
            port,
            download_duration_secs: download_duration,
            image_url,
            postgres: Postgres {
                host: postgres_host,
                port: postgres_port,
                db: postgres_db,
                user: postgres_user,
                password: postgres_password,
            },
        };

        info!(
            port = %config.port,
            download_duration_secs = %config.download_duration_secs,
            image_url = %config.image_url,
            postgres_host = %config.postgres.host,
            postgres_port = %config.postgres.port,
            postgres_db = %config.postgres.db,
            postgres_user = %config.postgres.user,
            "Configuration loaded successfully"
        );
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

