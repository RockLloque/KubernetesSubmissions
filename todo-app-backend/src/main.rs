use std::env;

use anyhow::Result;
use axum::{Router, routing::get};
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use tokio::time;
use tower_http::trace::TraceLayer;
use tracing::{error, info, instrument, Instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::core::config::Config;
use crate::core::models::AppState;
use crate::core::db;
use crate::routes::todos::{create_todo, get_todos};

mod core;
mod routes;

static IMAGE_PATH: &'static str = "/usr/local";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber with JSON formatting
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into())
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = Config::init()?;
    let port = config.port();
    let url = config.image_url();
    let duration = config.download_duration();

    // Initialize database connection pool
    let connection_string = config.connection_string();
    let db = db::init_pool(&connection_string).await?;

    task::spawn(
        async move {
            let image_path = env::var("IMAGE_PATH").unwrap_or(IMAGE_PATH.to_string());

            let client = Client::new();
            let mut interval = time::interval(duration);

            loop {
                if let Err(e) = download_image(&client, url.clone(), image_path.clone()).await {
                    error!(error = %e, "Failed to download image");
                }
                interval.tick().await;
            }
        }
        .instrument(tracing::info_span!("background_image_download"))
    );

    let state = AppState { db };

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    info!(port = %port, "Todo-App Server starting");
    axum::serve(listener, app).await?;
    Ok(())
}

#[instrument(skip(client), fields(url = %url, path = %image_path))]
async fn download_image(client: &Client, url: String, image_path: String) -> Result<()> {
    let response = client.get(url).send().await?.error_for_status()?;

    let bytes = response.bytes().await?;

    let mut file = File::create(format!("{}/image.jpg", image_path)).await?;
    file.write_all(&bytes).await?;

    info!(path = %image_path, "Image saved successfully");
    Ok(())
}

// Todo API endpoints
