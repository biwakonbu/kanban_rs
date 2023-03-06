use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use ulid::Ulid;

use crate::error::ApplicationResult;

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct Users {
    users: Vec<User>,
}

pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .with_state(pool)
}

pub async fn get_users(State(pool): State<MySqlPool>) -> ApplicationResult<impl IntoResponse> {
    let users = sqlx::query_as!(User, "SELECT id, name FROM users LIMIT 100")
        .fetch_all(&pool)
        .await?;

    Ok((StatusCode::OK, Json(users)))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

pub async fn create_user(
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
