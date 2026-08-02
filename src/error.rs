use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Asset does not exist")]
    AssetDoesNotExist,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User does not exist")]
    UserDoesNotExist,
    #[error("This username is already registered")]
    UsernameTaken,
    #[error("Missing authorization header")]
    MissingAuthorization,
    #[error("Invalid purchase: {0}")]
    InvalidPurchase(String),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Template(#[from] askama::Error),
    #[error(transparent)]
    Jwt(#[from] jwt_simple::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        let status = match self {
            Self::AssetDoesNotExist | Self::UserDoesNotExist => StatusCode::NOT_FOUND,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::UsernameTaken | Self::MissingAuthorization | Self::InvalidPurchase(_) => {
                StatusCode::BAD_REQUEST
            }
            Self::Database(_) | Self::Template(_) | Self::Jwt(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, Json(error_response)).into_response()
    }
}
