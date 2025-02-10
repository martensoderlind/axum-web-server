mod route;
mod handler;
mod model;
mod schema;

use std::sync::Arc;

use dotenv::dotenv;
use route::create_router;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

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

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    
    let app = create_router(Arc::new(AppState{db: pool.clone()})).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
