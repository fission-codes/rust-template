//! Opentelemetry tracing extensions and setup.

use crate::settings::Otel;
use anyhow::{anyhow, Result};
use const_format::formatcp;
use http::Uri;
use opentelemetry::{
    global, runtime,
    sdk::{self, propagation::TraceContextPropagator, trace::Tracer, Resource},
};
use opentelemetry_otlp::{TonicExporterBuilder, WithExportConfig};
use opentelemetry_semantic_conventions as otel_semcov;
use tonic::{metadata::MetadataMap, transport::ClientTlsConfig};

//const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_NAME: &str = "application";
const VERSION: &str = formatcp!("v{}", env!("CARGO_PKG_VERSION"));
const LANG: &str = "rust";

/// Initialize Opentelemetry tracing via the [OTLP protocol].
///
/// [OTLP protocol]: <https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/otlp.md>
pub fn init_tracer(settings: &Otel) -> Result<Tracer> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let resource = Resource::new(vec![
        otel_semcov::resource::SERVICE_NAME.string(PKG_NAME),
        otel_semcov::resource::SERVICE_VERSION.string(VERSION),
        otel_semcov::resource::TELEMETRY_SDK_LANGUAGE.string(LANG),
    ]);

    let endpoint = &settings.exporter_otlp_endpoint;

    let map = MetadataMap::with_capacity(2);

    let trace = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter(map, endpoint)?)
        .with_trace_config(sdk::trace::config().with_resource(resource))
        .install_batch(runtime::Tokio)
        .map_err(|e| anyhow!("failed to intialize tracer: {:#?}", e))?;

    Ok(trace)
}

fn exporter(map: MetadataMap, endpoint: &Uri) -> Result<TonicExporterBuilder> {
    // Over grpc transport
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint.to_string())
        .with_metadata(map);

    match endpoint.scheme_str() {
        Some("https") => {
            let host = endpoint
                .host()
                .ok_or_else(|| anyhow!("failed to parse host"))?;

            Ok(exporter.with_tls_config(ClientTlsConfig::new().domain_name(host.to_string())))
        }
        _ => Ok(exporter),
    }
}
