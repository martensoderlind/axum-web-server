mod db;
mod model;

use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use axum::{response::IntoResponse, routing::get, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::json;
use dotenv::dotenv;
async fn root() -> impl IntoResponse {
    const MESSAGE: &str = "Hello, World!!";
    let json_response = serde_json::json!({
        "status":"success",
        "message":MESSAGE
    });
    Json(json!({"message":MESSAGE}))
}

async fn user_id(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

pub struct AppState {
    db:Pool<Postgres>,
}
#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool= match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool)=> {
        println!("Successfully connected to database");
        pool
        }
        Err(err) => {
        println!("Failed to connect to database: {:?}",err);
        std::process::exit(1);
    }
    };
    let app_state = Arc::new(AppState { db: pool.clone() });

    let app = Router::new()
        .route("/", get(root))
        .route("/user/{id}", get(user_id))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
