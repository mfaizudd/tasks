use axum::{response::IntoResponse, Json};
use derive_more::Display;
use hyper::StatusCode;
use jsonwebtoken::errors::ErrorKind;
use serde::{ser::SerializeStruct, Serialize};
use thiserror::Error;
use validator::ValidationError;

use crate::response::Response;

#[derive(Serialize, Error, Debug, Display)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    ValidationError(#[from] ValidationError),
    AuthorizationError(String),
    InternalError(InternalError),
}

#[derive(Debug, Display)]
pub struct InternalError(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for InternalError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl Serialize for InternalError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("InternalError", 2)?;
        state.serialize_field("type", "INTERNAL_ERROR")?;
        state.serialize_field("value", &self.0.to_string())?;
        state.end()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::RowNotFound = err {
            return ApiError::NotFound("Resource not found".to_string());
        }
        ApiError::InternalError(err.into())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::InternalError(err.into())
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::InternalError(err.into())
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => {
                ApiError::AuthorizationError("Token expired".to_string())
            }
            ErrorKind::InvalidToken => ApiError::AuthorizationError("Invalid token".to_string()),
            ErrorKind::InvalidIssuer => ApiError::AuthorizationError("Invalid issuer".to_string()),
            ErrorKind::InvalidAudience => {
                ApiError::AuthorizationError("Invalid audience".to_string())
            }
            err => {
                println!("Error: {:?}", err);
                ApiError::AuthorizationError("Invalid token".to_string())
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::AuthorizationError(_) => StatusCode::UNAUTHORIZED,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body: Response<()> = Response::error(self, "An error has occured".to_string(), vec![]);
        (status, Json(body)).into_response()
    }
}
