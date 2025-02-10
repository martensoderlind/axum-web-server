use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}