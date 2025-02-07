use axum::http::StatusCode;
use axum::{extract::Path, extract::State, routing::get, Json, Router};
mod db;
use sqlx::PgPool;

async fn root() -> String {
    format!("hej")
}
async fn user(State(pool): State<PgPool>) -> Result<Json<Vec<db::User>>, (StatusCode, String)> {
    db::get_users(&pool).await.map(Json).map_err(|e| {
        eprintln!("Failed to get users: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

async fn user_id(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not load .env file");

    let pool = db::create_pool()
        .await
        .expect("failed creating database client pool");

    db::create_users_table(&pool)
        .await
        .expect("failed creating user table.");

    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(user))
        .route("/user/{id}", get(user_id))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
