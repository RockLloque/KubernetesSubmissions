use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use axum::{Router, extract::State, routing::get};
use dotenv::dotenv;
use reqwest::Client;
use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("3000".to_string()).to_string();

    task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(60 * 10)); // every 10 minutes
        let url = env::var("URL").unwrap_or("https://picsum.photos/1200".to_string());
        let image_path = env::var("IMAGE_PATH").unwrap_or("/tmp/kube".to_string());
        let client = Client::new();

        loop {
            interval.tick().await;
            if let Err(e) = download_image(&client, url.clone(), image_path.clone()).await {
                eprintln!("Failed to download image {}", e);
            }
        }
    });

    let app = Router::new().route("/", get(root).with_state(port.clone()));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    println!("Sever starting on port: {port}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root(State(port): State<String>) -> String {
    format!("Server started in port {}", port)
}

async fn download_image(client: &Client, url: String, image_path: String) -> Result<()> {
    let response = client.get(url).send().await?.error_for_status()?;

    let bytes = response.bytes().await?;

    let mut file = File::create(format!("{}/image.jpg", image_path))?;
    file.write_all(&bytes)?;

    println!("Image saved");
    Ok(())
}
