use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}