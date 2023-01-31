//! Middleware for runtime, [tower_http] extensions.

use crate::error::AppError;

use axum::response::{IntoResponse, Response};
use std::any::Any;

/// Middleware function for catching runtime panics, logging
/// them, and converting them into a `500 Internal Server` response.
pub fn catch_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    let err: AppError = anyhow::anyhow!(details).into();
    err.into_response()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::{parse_error, AppError};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::{ServiceBuilder, ServiceExt};
    use tower_http::catch_panic::CatchPanicLayer;

    #[tokio::test]
    async fn catch_panic_error() {
        let middleware = ServiceBuilder::new().layer(CatchPanicLayer::custom(catch_panic));

        let app = Router::new()
            .route("/", get(|| async { panic!("hi") }))
            .layer(middleware);

        let res = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let err = parse_error(res).await;

        assert_eq!(
            err,
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, Some("hi"))
        );
    }
}
