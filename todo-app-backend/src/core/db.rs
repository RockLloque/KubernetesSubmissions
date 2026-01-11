use anyhow::Result;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::{info, instrument};

use crate::core::models::{CreateTodo, Todo};

static ORDER_SPACING: i32 = 10_000;

/// Initialize database connection pool and run migrations
#[instrument(skip(connection_string), fields(connection_string = "***"))]
pub async fn init_pool(connection_string: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(connection_string)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database connection pool initialized and migrations completed");
    Ok(pool)
}

/// Get all todos ordered by order field
#[instrument(skip(pool))]
pub async fn get_todos(pool: &PgPool) -> Result<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
            SELECT 
                id, title, description, sorting_order 
            FROM 
                todos 
            ORDER BY 
                sorting_order ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

/// Create a new todo
#[instrument(skip(pool), fields(title = %todo.title))]
pub async fn create_todo(pool: &PgPool, todo: &CreateTodo) -> Result<Todo> {
    let last_order: Option<i32> = sqlx::query_scalar(
        r#"
      SELECT 
        sorting_order 
      FROM
        todos
      ORDER BY
        sorting_order DESC
      LIMIT
         1
    "#,
    )
    .fetch_optional(pool)
    .await?;

    let sorting_order = last_order.map_or(ORDER_SPACING, |o| o + ORDER_SPACING);
    let created_todo = sqlx::query_as::<_, Todo>(
        r#"
           INSERT INTO 
              todos (title, description, sorting_order)
           VALUES
              ($1, $2, $3)
           RETURNING 
              id, title, description, sorting_order
        "#,
    )
    .bind(&todo.title)
    .bind(&todo.description)
    .bind(sorting_order)
    .fetch_one(pool)
    .await?;

    Ok(created_todo)
}
