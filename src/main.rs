use axum::{
    routing::get,
    extract::Path,
    Router,
};

async fn root()->String{
    let s = String::from("root");
    return s
}
async fn user()->String{
    let s = String::from("All users");
    return s
}
async fn user_id(Path(id):Path<u32>)->String{
    format!("User ID: {}", id)
}
#[tokio::main]
async fn main() {
    let app =  Router::new()
        .route("/",get(root))
        .route("/user",get(user))
        .route("/user/{id}",get(user_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

