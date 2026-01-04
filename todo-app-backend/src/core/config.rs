use std::time::Duration;
use anyhow::Result;
use serde::{Deserialize, Deserializer};

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
    fn new(host: String, port: u16, db: String, user: String, password: String) -> Self {
        Self {
            host,
            port,
            db,
            user,
            password,
        }
    }

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

    #[serde(rename = "DOWNLOAD_DURATION", deserialize_with = "duration_from_secs")]
    download_duration: Duration, 

    #[serde(rename = "IMAGE_URL")]
    image_url: String,

    #[serde(flatten)]
    postgres: Postgres,
}

impl Config {
    pub fn init() -> Result<Self> {
        dotenvy::dotenv()?;

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
        self.download_duration
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

fn duration_from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
}
