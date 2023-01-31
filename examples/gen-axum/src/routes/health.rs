//! Healthcheck route.

use crate::error::AppResult;
use axum::{self, http::StatusCode};
use serde_json::json;

/// GET handler for checking service health.
#[utoipa::path(
    get,
    path = "/healthcheck",
    responses(
        (status = 200, description = "gen-axum healthy"),
        (status = 500, description = "gen-axum not healthy", body=AppError)
    )
)]
pub async fn healthcheck() -> AppResult<(StatusCode, axum::Json<serde_json::Value>)> {
    Ok((StatusCode::OK, axum::Json(json!({ "msg": "Healthy"}))))
}
