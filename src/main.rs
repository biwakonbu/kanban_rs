mod error;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::{env, time::Duration};
use ulid::Ulid;

use crate::error::ApplicationResult;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let users_router = Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user));

    let app = Router::new().nest("/users", users_router).with_state(pool);

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

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUser>,
) -> ApplicationResult<impl IntoResponse> {
    let user = User {
        id: Ulid::new().to_string(),
        name: payload.username,
    };

    sqlx::query!(
        "INSERT INTO users (id, name) values (?, ?)",
        &user.id,
        &user.name
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::OK, Json(user)))
}
