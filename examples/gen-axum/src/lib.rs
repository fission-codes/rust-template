#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! gen-axum

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
