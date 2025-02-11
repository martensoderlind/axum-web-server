use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        get_user_handler,create_users_handler,users_list_handler, delete_user_handler
    },
    AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/",get("users_list_handler"))
        .route("/users",get(users_list_handler))
        .route("/users",post(create_users_handler))
        .route(
            "/users/:id",
            get(get_user_handler)
                .delete(delete_user_handler),
        )
        .with_state(app_state)
}
