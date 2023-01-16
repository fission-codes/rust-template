//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::{health, ping},
};
use utoipa::OpenApi;

/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(health::healthcheck, ping::get),
        components(schemas(AppError)),
        tags(
            (name = "", description = "{{project-name}} service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
