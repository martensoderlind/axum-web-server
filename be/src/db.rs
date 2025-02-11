use serde::Serialize;
use sqlx::postgres::PgPool;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Environment variable error: {0}")]
    Environment(#[from] env::VarError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub async fn create_pool() -> Result<PgPool, DatabaseError> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    sqlx::query("SELECT 1").fetch_one(&pool).await?;
    Ok(pool)
}
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    id: i32,
    name: String,
}

pub async fn create_users_table(pool: &PgPool) -> Result<(), DatabaseError> {
    sqlx::query!(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )
        "
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, DatabaseError> {
    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    println!("users:{:?}", users);
    Ok(users)
}
