//! Adding trace information to [reqwest::Request]s.

use reqwest::{Request, Response};
use reqwest_middleware::Result;
use reqwest_tracing::{default_on_request_end, reqwest_otel_span, ReqwestOtelSpanBackend};
use std::time::Instant;
use task_local_extensions::Extensions;
use tracing::Span;

/// Latency string.
const LATENCY_FIELD: &str = "latency_ms";

/// Struct for extending [reqwest_tracing::TracingMiddleware].
#[derive(Debug)]
pub struct ExtendedTrace;

impl ReqwestOtelSpanBackend for ExtendedTrace {
    fn on_request_start(req: &Request, extension: &mut Extensions) -> Span {
        extension.insert(Instant::now());
        reqwest_otel_span!(
            name = "reqwest-http-request",
            req,
            latency_ms = tracing::field::Empty
        )
    }

    fn on_request_end(span: &Span, outcome: &Result<Response>, extension: &mut Extensions) {
        let elapsed_milliseconds = extension.get::<Instant>().unwrap().elapsed().as_millis() as i64;
        default_on_request_end(span, outcome);
        span.record(LATENCY_FIELD, elapsed_milliseconds);
    }
}
