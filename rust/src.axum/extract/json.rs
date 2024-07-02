//! JSON Extrator / Response replacement for [axum::extract::Json].

use async_trait::async_trait;
use axum::{
    body::{Bytes},
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use serde::{de::DeserializeOwned, Serialize};
use std::ops::{Deref, DerefMut};
use axum::extract::Request;
use axum::http::{header, StatusCode};
use axum_extra::headers::{HeaderMap, HeaderValue};
use tracing::warn;

use crate::error::AppError;

/// JSON Extractor / Response.
/// Built to replace axum::extract::Json, due to the manner of error response.
/// [axum::extract::Json] does not provide much useful information when json parsing fails.
/// This will parse the json using [`serde_path_to_error`] which adds much better
/// context on parsing errors.
///
/// When used as an extractor, it can deserialize request bodies into some type that
/// implements [serde::Deserialize]. The request will be rejected (and a [AppError] will
/// be returned) if:
///
/// - The request doesn't have a `Content-Type: application/json` (or similar) header.
/// - The body doesn't contain syntactically valid JSON.
/// - The body contains syntactically valid JSON but it couldn't be deserialized into the target
///   type.
/// - Buffering the request body fails.
///
/// See [AppError] for more details.
///
/// # Extractor example
///
/// ```rust,no_run
/// use std::net::SocketAddr;
/// use axum::{
///     response,
///     routing::post,
///     Router,
/// };
/// use {{crate_name}}::extract;
/// use serde::Deserialize;
///
/// use tokio::net::TcpListener;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// async fn create_user(extract::json::Json(payload): extract::json::Json<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # async {
/// # let port = 3000;
/// # let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
/// #     axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
/// #         .await
/// #         .unwrap();
/// # };
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Json<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if json_content_type(req.headers()) {
            let bytes = Bytes::from_request(req, state).await.map_err(|err| {
                warn!(
                    subject = "request",
                    category = "parsing",
                    "unable to parse request body {:#}",
                    err,
                );
                AppError::new(
                    http::StatusCode::BAD_REQUEST,
                    Some("Unable to parse request body"),
                )
            })?;
            let jd = &mut serde_json::Deserializer::from_slice(bytes.as_ref());
            let result: Result<T, _> = serde_path_to_error::deserialize(jd);
            match result {
                Ok(value) => Ok(Json(value)),
                Err(err) => {
                    let err_response = match err.inner().classify() {
                        serde_json::error::Category::Data => {
                            warn!(
                                subject = "request",
                                category = "parsing",
                                json_error_path = ?err.path().to_string(),
                                "failed to deserialize the JSON body into the target type, json error: {:#}",
                                err.inner()
                            );
                            AppError::new(
                                http::StatusCode::UNPROCESSABLE_ENTITY,
                                Some(format!(
                                    "failed to deserialize the JSON body into the target type, json error path: {}; json error {:#}",
                                    err.path(),
                                    err.inner()
                                ))
                            )
                        }
                        _ => {
                            warn!(
                                subject = "request",
                                category = "parsing",
                                "failed to parse the request body as JSON; json error {:#}",
                                err.inner()
                            );
                            AppError::new(
                                http::StatusCode::BAD_REQUEST,
                                Some(format!(
                                    "failed to parse the request body as JSON; json error {:#}",
                                    err.inner()
                                )),
                            )
                        }
                    };
                    Err(err_response)
                }
            }
        } else {
            Err(AppError::new(
                http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Some("Expected request with `Content-Type: application/json`"),
            ))
        }
    }
}

fn json_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|content_type| content_type.to_str().ok())
        .and_then(|content_type| content_type.parse::<mime::Mime>().ok())
        .map(|mime| {
            mime.type_() == "application"
                && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"))
        })
        .unwrap_or(false)
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                bytes,
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use super::*;
    use axum::routing::{get, Router};
    use axum::http::Request;
    use serde::Deserialize;
    use tower::ServiceExt;

    #[derive(Debug, Deserialize)]
    struct Input {
        foo: String,
    }

    #[tokio::test]
    async fn deserialize_body() {
        let app = Router::new().route(
            "/",
            get(|input: Json<Input>| async { (StatusCode::OK, input.0.foo) }),
        );
        let contents = r#"{ "foo": "bar" }"#;
        let req_body: Body = contents.into();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(req_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(),usize::MAX).await.unwrap();
        let body_text = std::str::from_utf8(&body[..]).unwrap();
        dbg!(body_text);
        assert_eq!(body_text, "bar");
    }

    #[tokio::test]
    async fn consume_body_to_json_requires_json_content_type() {
        let app = Router::new().route(
            "/",
            get(|input: Json<Input>| async { (StatusCode::OK, input.0.foo) }),
        );
        let contents = r#"{ "foo": "bar" }"#;
        let req_body: Body = contents.into();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Content-Type", "application/text")
                    .body(req_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn json_content_types() {
        async fn valid_json_content_type(content_type: &str) -> bool {
            dbg!(content_type);

            let app = Router::new().route(
                "/",
                get(|input: Json<Input>| async { (StatusCode::OK, input.0.foo) }),
            );
            let contents = r#"{ "foo": "bar" }"#;
            let req_body: Body = contents.into();
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/")
                        .header("Content-Type", content_type)
                        .body(req_body)
                        .unwrap(),
                )
                .await
                .unwrap();

            response.status() == StatusCode::OK
        }

        assert!(valid_json_content_type("application/json").await);
        assert!(valid_json_content_type("application/json; charset=utf-8").await);
        assert!(valid_json_content_type("application/json;charset=utf-8").await);
        assert!(valid_json_content_type("application/cloudevents+json").await);
        assert!(!valid_json_content_type("text/json").await);
    }

    #[tokio::test]
    async fn invalid_json_syntax() {
        let app = Router::new().route(
            "/",
            get(|input: Json<Input>| async { (StatusCode::OK, input.0.foo) }),
        );
        let contents = "{";
        let req_body: Body = contents.into();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(req_body)
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
