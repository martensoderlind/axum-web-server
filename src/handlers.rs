use std::sync::Arc;

use axum::{extract::{State, path, Query}, http::StatusCode, response::IntoResponse, Json, Router};
use axum::extract::Path;
use serde_json::json;
use uuid::Uuid;
use crate::{
    model::UserModel,
    schema::{CreateUserSchema, UpdateUserSchema, FilterOptions},
    AppState
};

pub async fn users_list_handler(
    opt:Option<Query<FilterOptions>>,
    State(data):State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opt) = opt.unwrap_or_default();
    let limit = opt.limit.unwrap_or(10);
    let offset = (opt.page.unwrap_or(1))-1*limit;

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status":"fail",
            "message": "Something went wrong while fetching the users."
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }
    let users = query_result.unwrap();

    let json_response = serde_json::json!({
        "status":"ok",
        "result": users.len(),
        "users": users,
    });
    Ok(Json(json_response))
}

pub async fn create_users_handler(State(data):State<Arc<AppState>>,
    Json(body):Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        UserModel,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        body.name.to_string(),
        body.email.to_string()
    )
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(user) => {
            let user_response = json!({
                "status":"success",
                "data": json!({
                    "user": user
                }),
            });
            Ok(Json(user_response))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "status":"fail",
                "message": "Something went wrong while creating a user."
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn get_user_handler(
    Path(id):Path<Uuid>,
    State(data):State<Arc<AppState>>,
)->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE id = $1",
        id
    )
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({
                    "user":user
                })
            })
            return Ok(Json(user_response));
        }
        Err(err) => {
            let error_message = serde_json::json!({
                "status":"fail",
                "message": format!("User with id:{} could not be found", id)
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)));
        }
    }
}