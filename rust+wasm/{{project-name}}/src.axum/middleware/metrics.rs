//! Middleware for tracking metrics on each [axum::http::Request].

use crate::middleware::request_ext::RequestExt;
use axum::{http::Request, middleware::Next, response::IntoResponse};
use std::time::Instant;

/// Middleware function called to track (and update) http metrics when a route
/// is requested.
pub async fn track<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();

    let method = req.method().clone();
    let path = req.path();

    let res = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let status = res.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("request_path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_request_duration_seconds", latency, &labels);

    res
}
