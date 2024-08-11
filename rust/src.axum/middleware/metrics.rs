//! Middleware for tracking metrics on each [axum::http::Request].

use crate::middleware::request_ext::RequestExt;
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use std::time::Instant;

/// Middleware function called to track (and update) http metrics when a route
/// is requested.
pub async fn track(req: Request<Body>, next: Next) -> impl IntoResponse {
    let start = Instant::now();

    let path = req.path().to_string();

    let res = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let status = res.status().as_u16();

    metrics::counter!("http_requests_total").increment(1);
    metrics::counter!("http_requests_total", "request_path" => path.clone() ).increment(1);
    metrics::counter!("http_requests_total", "request_path" => path.clone(), "status" => status.to_string() ).increment(1);
    metrics::histogram!("http_request_duration_seconds").record(latency);
    metrics::histogram!("http_request_duration_seconds", "request_path" => path.clone())
        .record(latency);
    metrics::histogram!("http_request_duration_seconds", "request_path" => path.clone(), "status" => status.to_string()).record(latency);
    res
}
