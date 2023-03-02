use axum::{http::StatusCode, routing::get, Json, Router};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url);

    let users_router = Router::new().route("/", get(get_users));

    let app = Router::new().nest("/users", users_router);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct Users {
    users: Vec<User>,
}

async fn get_users() -> (StatusCode, Json<Users>) {
    let user = User {
        id: "".to_string(),
        name: "".to_string(),
    };
    let users = Users { users: vec![user] };
    (StatusCode::OK, Json(users))
}
