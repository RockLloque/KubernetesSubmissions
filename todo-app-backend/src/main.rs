use std::time::Duration;
use std::{env, fs};

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use dotenv::dotenv;
use reqwest::{Client, header};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use tokio::time;
use tower_http::cors::{CorsLayer, Any};

use crate::core::models::AppState;
use crate::routes::todos::{create_todo, get_todos};

mod core;
mod routes;

static IMAGE_PATH: &'static str = "/usr/local";
static TODOS_PATH: &'static str = "./todos";

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

    let todos_path = env::var("TODOS_PATH").unwrap_or(TODOS_PATH.to_string());

    let state = AppState { todos_path };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos).post(create_todo))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    println!("Todo-App Server starting on port: {port}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn download_image(client: &Client, url: String, image_path: String) -> Result<()> {
    let response = client.get(url).send().await?.error_for_status()?;

    let bytes = response.bytes().await?;

    let mut file = File::create(format!("{}/image.jpg", image_path)).await?;
    file.write_all(&bytes).await?;

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

// Todo API endpoints
