use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("SQL Failed: {0:?}")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            Self::SqlxError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: err.to_string(),
                }),
            ),
        }
        .into_response()
    }
}

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;
