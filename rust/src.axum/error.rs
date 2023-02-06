//! Generic result/error resprentation(s).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::{Deserialize, Serialize};
use tracing::warn;
use ulid::Ulid;
use utoipa::ToSchema;

/// Standard return type out of routes / handlers
pub type AppResult<T> = std::result::Result<T, AppError>;

/// Encodes [JSONAPI error object responses](https://jsonapi.org/examples/#error-objects).
///
/// JSONAPI error object -  ALL Fields are technically optional.
///
/// This struct uses the following guidelines:
///
/// 1. Always encode the StatusCode of the response
/// 2. Set the title to the `canonical_reason` of the status code.
///    According to spec, this should NOT change over time.
/// 3. For unrecoverable errors, encode the detail as the to_string of the error
///
/// Other fields not currently captured (but can be added)
///
/// - id - a unique identifier for the problem
/// - links - a link object with further information about the problem
/// - source - a JSON pointer indicating a problem in the request json OR
///   a parameter specifying a problematic query parameter
/// - meta - a meta object containing arbitrary information about the error
#[derive(ToSchema, thiserror::Error, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct AppError {
    #[schema(value_type = u16, example = 200)]
    #[serde(with = "crate::error::serde_status_code")]
    status: StatusCode,
    detail: Option<String>,
    title: Option<String>,
}

impl AppError {
    /// New instance of [AppError].
    pub fn new<M: ToString>(status_code: StatusCode, message: Option<M>) -> AppError {
        Self {
            status: status_code,
            title: Self::canonical_reason_to_string(&status_code),
            detail: message.map(|m| m.to_string()),
        }
    }

    /// [AppError] for [StatusCode::NOT_FOUND].
    pub fn not_found(id: Ulid) -> AppError {
        Self::new(
            StatusCode::NOT_FOUND,
            Some(format!("Entity with id {id} not found")),
        )
    }

    fn canonical_reason_to_string(status_code: &StatusCode) -> Option<String> {
        status_code.canonical_reason().map(|r| r.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// Error in JSON API response format.
pub struct ErrorResponse {
    errors: Vec<AppError>,
}

impl From<AppError> for ErrorResponse {
    fn from(e: AppError) -> Self {
        Self { errors: vec![e] }
    }
}

impl From<AppError> for (StatusCode, Json<ErrorResponse>) {
    fn from(app_error: AppError) -> Self {
        (app_error.status, Json(app_error.into()))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_response: (StatusCode, Json<ErrorResponse>) = self.into();
        error_response.into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        warn!(
            subject = "app_error",
            category = "app_error",
            "encountered unexpected error {:#}: {:#}",
            err,
            err.backtrace()
        );
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            title: StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .map(|r| r.to_string()),
            detail: Some(err.to_string()),
        }
    }
}

/// Serialize/Deserializer for status codes.
///
/// This is needed because status code according to JSON API spec must
/// be the status code as a STRING.
///
/// We could have used http_serde, but it encodes the status code as a NUMBER.
pub mod serde_status_code {
    use http::StatusCode;
    use serde::{de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};

    /// Serialize [StatusCode]s.
    pub fn serialize<S: Serializer>(status: &StatusCode, ser: S) -> Result<S::Ok, S::Error> {
        String::serialize(&status.as_u16().to_string(), ser)
    }

    /// Deserialize [StatusCode]s.
    pub fn deserialize<'de, D>(de: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(de)?;
        StatusCode::from_bytes(str.as_bytes()).map_err(|_| {
            serde::de::Error::invalid_value(
                Unexpected::Str(str.as_str()),
                &"A valid http status code",
            )
        })
    }
}

// Needed to support thiserror::Error, outputs debug for AppError
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
/// Parse the app error out of the json body
pub async fn parse_error(response: Response) -> AppError {
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let mut err_response: ErrorResponse = serde_json::from_slice(&body_bytes).unwrap();
    err_response.errors.remove(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::StatusCode, response::IntoResponse};
    use ulid::Ulid;

    #[test]
    fn test_from_anyhow_error() {
        let err: AppError = anyhow::anyhow!("FAIL").into();
        assert_eq!(err.detail.unwrap(), "FAIL".to_string());
        assert_eq!(
            err.title,
            StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .map(|r| r.to_string())
        );

        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_not_found() {
        let id = Ulid::new();
        let err = AppError::not_found(id);

        assert_eq!(err.status, StatusCode::NOT_FOUND);
        assert_eq!(
            err.title,
            StatusCode::NOT_FOUND
                .canonical_reason()
                .map(|r| r.to_string())
        );
        assert_eq!(
            err.detail.unwrap(),
            format!("Entity with id {id} not found")
        );
    }

    #[tokio::test]
    async fn test_json_api_error_response() {
        // verify that our json api response complies with the standard
        let id = Ulid::new();
        let err = AppError::not_found(id);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let err = parse_error(response).await;

        // Check that the result is all good
        assert_eq!(err.status, StatusCode::NOT_FOUND);
        assert_eq!(
            err.title,
            StatusCode::NOT_FOUND
                .canonical_reason()
                .map(|r| r.to_string())
        );
        assert_eq!(
            err.detail.unwrap(),
            format!("Entity with id {id} not found")
        );
    }
}
