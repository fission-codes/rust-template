//! Middleware for generating [ulid::Ulid]s on requests.

use axum::http::Request;
use tower_http::request_id::{MakeRequestId, RequestId};
use ulid::Ulid;

/// Make/generate ulid on requests.
#[derive(Copy, Clone, Debug)]
pub struct MakeRequestUlid;

/// Implement the trait for producing a request ID from the incoming request.
/// In our case, we want to generate a new UUID that we can associate with a single request.
impl MakeRequestId for MakeRequestUlid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let req_id = Ulid::new().to_string().parse();
        match req_id {
            Ok(id) => Some(RequestId::new(id)),
            _ => None,
        }
    }
}
