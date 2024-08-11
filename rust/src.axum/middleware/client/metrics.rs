//! Middleware for tracking metrics on each client [reqwest::Request].

use http::Extensions;
use reqwest_middleware::Middleware as ReqwestMiddleware;
use std::time::Instant;

const OK: &str = "ok";
const ERROR: &str = "error";
const MIDDLEWARE_ERROR: &str = "middleware_error";
const NONE: &str = "none";
const RESULT: &str = "result";
const STATUS: &str = "status";

/// Metrics struct for use as part of middleware.
#[derive(Debug)]
pub struct Metrics {
    /// Client name for metric(s) gathering.
    pub name: String,
}

#[async_trait::async_trait]
impl ReqwestMiddleware for Metrics {
    async fn handle(
        &self,
        request: reqwest::Request,
        extensions: &mut Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> Result<reqwest::Response, reqwest_middleware::Error> {
        let now = Instant::now();

        let url = request.url().clone();
        let request_path = url.path().to_string();
        let method = request.method().clone();

        let result = next.run(request, extensions).await;
        let latency = now.elapsed().as_secs_f64();

        let labels = vec![
            ("client", self.name.to_string()),
            ("method", method.to_string()),
            ("request_path", request_path.clone()),
        ];

        let extended_labels = extend_labels_for_response(labels, &result);

        metrics::counter!("client_http_requests_total").increment(1u64);
        metrics::counter!("client_http_requests_total", &extended_labels).increment(1u64);
        metrics::counter!("client_http_requests_total", "client" => self.name.to_string())
            .increment(1u64);
        metrics::counter!("client_http_requests_total", "client" => self.name.to_string(), "request_path" => request_path.clone()).increment(1u64);
        metrics::histogram!("client_http_request_duration_seconds").record(latency);
        metrics::histogram!("client_http_request_duration_seconds", &extended_labels)
            .record(latency);
        metrics::histogram!("client_http_request_duration_seconds", "client" => self.name.to_string()).record(latency);
        metrics::histogram!("client_http_request_duration_seconds", "client" => self.name.to_string(), "request_path" => request_path.clone()).record(latency);

        result
    }
}

/// Extend a set of metrics label tuples with dynamic properties
/// around reqwest responses for `result` and `status` fields.
pub fn extend_labels_for_response<'a>(
    mut labels: Vec<(&'a str, String)>,
    result: &Result<reqwest::Response, reqwest_middleware::Error>,
) -> Vec<(&'a str, String)> {
    match result {
        Ok(ref success) => {
            match success.status().as_u16() {
                200..=299 => labels.push((RESULT, OK.to_string())),
                _ => labels.push((RESULT, ERROR.to_string())),
            }

            labels.push((STATUS, success.status().as_u16().to_string()));
        }
        Err(reqwest_middleware::Error::Reqwest(ref err)) => {
            labels.push((RESULT, ERROR.to_string()));
            labels.push((
                STATUS,
                err.status()
                    .map(|status| status.as_u16().to_string())
                    .unwrap_or_else(|| NONE.to_string()),
            ));
        }
        Err(reqwest_middleware::Error::Middleware(ref _err)) => {
            labels.push((RESULT, MIDDLEWARE_ERROR.to_string()));
            labels.push((STATUS, NONE.to_string()));
        }
    };

    labels
}
