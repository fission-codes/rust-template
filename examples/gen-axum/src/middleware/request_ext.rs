//! Middleware for additional [axum::http::Request] methods.

use axum::{
    extract::{MatchedPath, OriginalUri},
    http::Request,
};

/// Trait for extra methods on [`Request`](axum::http::Request)
pub(crate) trait RequestExt<T> {
    /// Parse request path on the request.
    fn path(&self) -> String;
}

impl<T> RequestExt<T> for Request<T> {
    fn path(&self) -> String {
        if let Some(matched_path) = self.extensions().get::<MatchedPath>() {
            matched_path.as_str().to_string()
        } else if let Some(uri) = self.extensions().get::<OriginalUri>() {
            uri.0.path().to_string()
        } else {
            self.uri().path().to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::Request;

    #[test]
    fn parse_path() {
        let mut req1: Request<()> = Request::default();
        *req1.uri_mut() = "https://www.rust-lang.org/users/:id".parse().unwrap();
        assert_eq!(req1.path(), "/users/:id");

        let mut req2: Request<()> = Request::default();
        *req2.uri_mut() = "https://www.rust-lang.org/api/users".parse().unwrap();
        assert_eq!(req2.path(), "/api/users");

        let mut req3: Request<()> = Request::default();
        *req3.uri_mut() = "/api/users".parse().unwrap();
        assert_eq!(req3.path(), "/api/users");
    }
}
