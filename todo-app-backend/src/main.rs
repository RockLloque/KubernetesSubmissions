use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{env, fs};

use anyhow::Result;
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use dotenv::dotenv;
use reqwest::{Client, StatusCode, header};
use tokio::task;
use tokio::time;

static IMAGE_PATH: &'static str = "/usr/local";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("3000".to_string()).to_string();

    task::spawn(async {
        let url = env::var("URL").unwrap_or("https://picsum.photos/1200".to_string());
        let image_path = env::var("IMAGE_PATH").unwrap_or(IMAGE_PATH.to_string());
        let duration: u64 = env::var("DOWNLOAD_DURATION")
            .map(|s| s.parse().unwrap_or(60 * 10))
            .unwrap_or(60 * 10);
        println!("Download duration: {duration}");

        let client = Client::new();
        let mut interval = time::interval(Duration::from_secs(duration)); // every 10 minutes

        loop {
            if let Err(e) = download_image(&client, url.clone(), image_path.clone()).await {
                eprintln!("Failed to download image {}", e);
            }
            interval.tick().await;
        }
    });

    let app = Router::new().route("/", get(root).with_state(port.clone()));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    println!("Todo-App Server starting on port: {port}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn download_image(client: &Client, url: String, image_path: String) -> Result<()> {
    let response = client.get(url).send().await?.error_for_status()?;

    let bytes = response.bytes().await?;

    let mut file = File::create(format!("{}/image.jpg", image_path))?;
    file.write_all(&bytes)?;

    println!("Image saved");
    Ok(())
}

async fn root() -> impl IntoResponse {
    let image_path = env::var("IMAGE_PATH").unwrap_or(IMAGE_PATH.to_string());
    let image_file = format!("{}/image.jpg", image_path);

    let image_content = if fs::metadata(&image_file).is_ok() {
        format!(
            r#"<img src="{}" alt="Latest Image" style="max-width: 100%; height: auto;">"#,
            image_file
        )
    } else {
        format!(r#"<p>Could not find image under {}</p>"#, image_file)
    };

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Todo App</title>
            <style>
                .todo-container {{
                    display: flex;
                    gap: 10px;
                    margin: 20px 0;
                }}
            </style>
        </head>
        <body>
            <h1>Todo App</h1>
            {}
            <div class="todo-container">
                <input type="text" maxlength="140" placeholder="Enter todo (max 140 characters)">
                <button>Create todo</button>
            </div>
        </body>
        </html>
        "#,
        image_content
    );

    (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], html).into_response()
}
