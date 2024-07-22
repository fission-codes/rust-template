//! Middleware for logging requests/responses for server and client calls.

use crate::{error::AppError, middleware::request_ext::RequestExt, settings::AppEnvironment};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use reqwest_middleware::Middleware as ReqwestMiddleware;
use task_local_extensions::Extensions;
use tracing::{debug, info, warn};

/// Generic "null" field for unset logs/fields.
const NULL: &str = "null";
/// None field.
const NONE: &str = "none";
/// Request identifier field.
const REQUEST_ID: &str = "request_id";

/// Middleware function for logging request and response body data.
pub async fn log_request_response<L: RequestResponseLogger>(
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let req = L::log_request(request).await?;
    let path = req.path();
    let res = next.run(req).await;
    let res = L::log_response(res, path).await;
    Ok(res)
}

/// Debug-only Request/Response Logger.
#[derive(Debug)]
pub struct DebugOnlyLogger;
/// Request/Response Logger.
#[derive(Debug)]
pub struct Logger;

/// Trait for request/response for logging.
/// Involves turning the request into bytes and re-forming it.
#[async_trait]
pub trait RequestResponseLogger {
    /// Log requests.
    ///
    /// Note: always on for debugging.
    async fn log_request(request: Request<Body>) -> Result<Request<Body>, AppError> {
        let path = request.path();
        let (parts, body) = request.into_parts();

        debug!(
            subject = "request",
            category="http.request",
            msg = "started processing request",
            request_path = %path,
            query_string = parts.uri.query());

        let bytes = buffer("Request", body).await?;
        if let Ok(body) = std::str::from_utf8(&bytes) {
            debug!(subject="request", category="http.request", body=?body, request_path=%path);
        }
        let req = Request::from_parts(parts, Body::from(bytes));
        Ok(req)
    }

    /// Log responses at different levels based on [StatusCode].
    async fn log_response(
        response: Response<Body>,
        path: String,
    ) -> Result<Response<Body>, AppError> {
        let status_code = response.status().as_u16();
        let headers = response.headers().clone();

        match status_code {
            200..=299 => {
                debug!(
                    subject = "response",
                    category="http.response",
                    status = ?status_code,
                    response_headers = ?headers,
                    "finished processing request")
            }
            500..=599 => {
                warn!(
                    subject = "response",
                    category="http.response",
                    status = ?status_code,
                    response_headers = ?headers,
                    "finished processing request")
            }
            _ => {
                info!(
                    subject = "response",
                    category="http.response",
                    status = ?status_code,
                    response_headers = ?headers,
                    "finished processing request")
            }
        }

        let (parts, body) = response.into_parts();

        let bytes = buffer("Response", body).await?;
        if let Ok(body) = std::str::from_utf8(&bytes) {
            debug!(subject="response", category="http.response", body=?body, request_path=%path);
        }
        let res = Response::from_parts(parts, Body::from(bytes));
        Ok(res)
    }
}

#[async_trait]
impl RequestResponseLogger for DebugOnlyLogger {}

#[async_trait]
impl RequestResponseLogger for Logger {
    async fn log_request(request: Request<Body>) -> Result<Request<Body>, AppError> {
        let path = request.path();
        let (parts, body) = request.into_parts();

        match parts.extensions.get::<AppEnvironment>() {
            Some(&AppEnvironment::Local) | Some(&AppEnvironment::Dev) => info!(
                    subject = "request",
                    category="http.request",
                    msg = "started processing request",
                    request_id = parts
                        .headers
                        .get(REQUEST_ID)
                        .map(|h| h.to_str().unwrap_or(NULL)),
                    request_path = %path,
                    query_string = parts.uri.query(),
                    authorization = parts
                        .headers
                        .get(http::header::AUTHORIZATION.as_str())
                        .map(|h| h.to_str().unwrap_or(NULL))
                    .unwrap_or(NULL)),
            _ => {
                info!(
                    subject = "request",
                    category="http.request",
                    msg = "started processing request",
                    request_id = parts
                        .headers
                        .get(REQUEST_ID)
                        .map(|h| h.to_str().unwrap_or(NULL)),
                    request_path = %path,
                    query_string = parts.uri.query(),
                    authorization= parts
                        .headers
                        .get(http::header::AUTHORIZATION.as_str())
                        .map(|h| if h.is_sensitive() {
                            "<redacted>"
                        }  else {
                            h.to_str().unwrap_or(NULL)
                        })
                    .unwrap_or(NULL))
            }
        };

        let bytes = buffer("Request", body).await?;

        if let Ok(body) = std::str::from_utf8(&bytes) {
            debug!(subject="request", category="http.request", body=?body);
        }

        let req = Request::from_parts(parts, Body::from(bytes));
        Ok(req)
    }
}

#[async_trait]
impl ReqwestMiddleware for Logger {
    async fn handle(
        &self,
        request: reqwest::Request,
        extensions: &mut Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> Result<reqwest::Response, reqwest_middleware::Error> {
        log_reqwest(&request, extensions);
        let url = request.url().clone();
        let _ = extensions.insert(url);
        match next.run(request, extensions).await {
            Ok(success) => {
                let response = log_reqwest_response(success, extensions).await?;
                Ok(response)
            }
            Err(reqwest_middleware::Error::Reqwest(err)) => {
                log_reqwest_error(&err, extensions)?;
                Err(reqwest_middleware::Error::Reqwest(err))
            }
            Err(reqwest_middleware::Error::Middleware(err)) => {
                log_middleware_error(&err, extensions)?;
                Err(reqwest_middleware::Error::Middleware(err))
            }
        }
    }
}

fn log_reqwest(request: &reqwest::Request, extensions: &mut Extensions) {
    let user_agent = request
        .headers()
        .get(http::header::USER_AGENT)
        .map(|h| h.to_str().unwrap_or(NULL))
        .unwrap_or(NULL);

    let host_hdr = request
        .headers()
        .get(http::header::HOST)
        .map(|h| h.to_str().unwrap_or(NULL))
        .unwrap_or(NULL);
    let host = request.url().host_str().unwrap_or(host_hdr);

    match extensions.get::<AppEnvironment>() {
        Some(&AppEnvironment::Local) | Some(&AppEnvironment::Dev) => {
            info!(
                subject = "client.request",
                category="http.request",
                client.method = %request.method(),
                client.url = %request.url(),
                client.host = host,
                client.request_path = request.url().path(),
                client.query_string = request.url().query(),
                client.user_agent = user_agent,
                client.version = ?request.version(),
                client.authorization= request
                    .headers()
                    .get(http::header::AUTHORIZATION.as_str())
                    .map(|h| h.to_str().unwrap_or(NULL))
                    .unwrap_or(NULL),
                "started processing client request")
        }
        _ => {
            info!(
                subject = "client.request",
                category="http.request",
                client.method = %request.method(),
                client.url = %request.url(),
                client.host = host,
                client.request_path = request.url().path(),
                client.query_string = request.url().query(),
                client.user_agent = user_agent,
                client.version = ?request.version(),
                client.authorization= request
                    .headers()
                    .get(http::header::AUTHORIZATION.as_str())
                    .map(|_h| "<redacted>")
                    .unwrap_or(NULL),
                "started processing client request")
        }
    }
}

async fn log_reqwest_response(
    response: reqwest::Response,
    extensions: &mut Extensions,
) -> Result<reqwest::Response> {
    /// Turn reqwest body, headers, status, and version into
    /// a generic [`http::Response`] and to capture body + parts,
    /// and then turn it back into a [`reqwest::Response`].
    ///
    /// For logging body information, the original response is
    /// eliminated, and a new one is formed. There's no way to
    /// from a Response::from_parts as in http/axum.
    async fn into_reqwest_response(
        body: reqwest::Body,
        headers: reqwest::header::HeaderMap,
        status_code: u16,
        version: reqwest::Version,
    ) -> Result<reqwest::Response> {
        let mut builder = http::Response::builder()
            .status(status_code)
            .version(version);

        let headers_iter = headers.into_iter();
        let headers = builder
            .headers_mut()
            .ok_or_else(|| anyhow!("failed to convert response headers"))?;

        headers.extend(headers_iter);

        let res = builder.body(body)?;
        Ok(reqwest::Response::from(res))
    }

    let url = extensions
        .get::<reqwest::Url>()
        .ok_or_else(|| anyhow!("failed to find Url extension"))?;

    let status_code = response.status().as_u16();

    let post_log_response = match status_code {
        400..=599 => {
            let version = response.version();
            let headers = response.headers().clone();
            let bytes = response.bytes().await?;
            if let Ok(body) = std::str::from_utf8(&bytes) {
                warn!(
                    subject = "client.response",
                    category="http.response",
                    body = ?body,
                    client.status = ?status_code,
                    client.response_headers = ?headers,
                    client.url = %url,
                    client.request_path = url.path(),
                    "error while processing client request");
            }

            let body = reqwest::Body::from(bytes);
            into_reqwest_response(body, headers, status_code, version).await?
        }
        _ => response,
    };

    Ok(post_log_response)
}

fn log_reqwest_error(error: &reqwest::Error, extensions: &mut Extensions) -> Result<()> {
    let url = extensions
        .get::<reqwest::Url>()
        .ok_or_else(|| anyhow!("failed to find Url extension"))?;

    warn!(
        subject = "client.response",
        category="http.response",
        client.error = format!("{:#?}", error.to_string()),
        client.request_path = url.path(),
        client.status = ?error.status().map(|status_code| status_code.as_u16().to_string()).unwrap_or_else(|| NONE.to_string()),
        client.url = %url,
        "error processing client request");

    Ok(())
}

fn log_middleware_error(error: &anyhow::Error, extensions: &mut Extensions) -> Result<()> {
    let url = extensions
        .get::<reqwest::Url>()
        .ok_or_else(|| anyhow!("failed to find Url extension"))?;

    warn!(
        subject = "client.response",
        category="http.response",
        error = format!("{:#?}", error.to_string()),
        client.url = %url,
        client.request_path = url.path(),
        client.status = NONE,
        "error processing client request within {{project-name}} middleware");

    Ok(())
}

async fn buffer(direction: &str, body: Body) -> Result<Bytes, anyhow::Error> {
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => anyhow::bail!(AppError::new(
            StatusCode::BAD_REQUEST,
            Some(format!("failed to read {direction} body: {err}")),
        )),
    };

    Ok(bytes)
}
