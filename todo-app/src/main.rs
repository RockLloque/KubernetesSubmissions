use std::env;

use anyhow::Result;
use axum::{Router, extract::State, routing::get};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("3000".to_string()).to_string();
    let app = Router::new().route("/", get(root).with_state(port.clone()));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn root(State(port): State<String>) -> String {
    format!("Server started in port {}", port)
}
