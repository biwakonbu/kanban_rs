mod api;
mod error;

use crate::api::user;
use axum::{
    Router,
};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, time::Duration};

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

    let app = Router::new()
        .nest("/users", user::router(pool.clone()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
