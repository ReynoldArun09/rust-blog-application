use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),
}

impl IntoResponse for AppError {
    fn into_reponse(self) -> Response {
        let status = match self {
            AppError::BadRequest(_) => StatusCode::BadRequest,
            AppError::Unauthorized(_) => StatusCode::Unauthorized,
            AppError::NotFound(_) => StatusCode::NotFound,
            _ => StatusCode::InternalServerError,
        };

        let body = Json(json!({ "error: self.to_String()" }));
        (status, body).into_reponse()
    }
}
