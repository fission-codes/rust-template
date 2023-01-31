//! Fallback routes.

use crate::error::AppError;
use axum::http::StatusCode;

/// 404 fallback.
pub async fn notfound_404() -> AppError {
    AppError::new(StatusCode::NOT_FOUND, Some("Route does not exist!"))
}
