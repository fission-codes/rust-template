#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub)]

//! {{project-name}}

pub mod docs;
pub mod error;
pub mod extract;
pub mod headers;
pub mod metrics;
pub mod middleware;
pub mod router;
pub mod routes;
pub mod settings;
pub mod tracer;
pub mod tracing_layers;
{% if bench %}
/// Test utilities.
#[cfg(any(test, feature = "test_utils"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test_utils")))]
pub mod test_utils;
/// Add two integers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}{% endif %}
