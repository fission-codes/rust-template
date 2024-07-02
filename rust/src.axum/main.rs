//! {{project-name}}

use anyhow::Result;
use axum::{extract::Extension, routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use std::{
    future::ready,
    io,
    net::SocketAddr,
    time::Duration,
};
use std::iter::once;
use headers::HeaderName;
use http::header;
use tokio::net::TcpListener;
use tokio::signal;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer, sensitive_headers::SetSensitiveHeadersLayer,
    timeout::TimeoutLayer, ServiceBuilderExt,
};
use tracing::info;
use tracing_subscriber::{
    filter::{dynamic_filter_fn, filter_fn, LevelFilter},
    prelude::*,
    EnvFilter,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use {{crate_name}}::{
    docs::ApiDoc,
    metrics::{process, prom::setup_metrics_recorder},
    middleware::{self, request_ulid::MakeRequestUlid, runtime},
    router,
    routes::fallback::notfound_404,
    settings::{Otel, Settings},
    tracer::init_tracer,
    tracing_layers::{
        format_layer::LogFmtLayer,
        metrics_layer::{MetricsLayer, METRIC_META_PREFIX},
        storage_layer::StorageLayer,
    },
};

/// Request identifier field.
const REQUEST_ID: &str = "request_id";

#[tokio::main]
async fn main() -> Result<()> {
    let (stdout_writer, _stdout_guard) = tracing_appender::non_blocking(io::stdout());

    let settings = Settings::load()?;
    setup_tracing(stdout_writer, settings.otel())?;

    info!(
        subject = "app_settings",
        category = "init",
        "starting with settings: {:?}",
        settings,
    );

    let env = settings.environment();
    let recorder_handle = setup_metrics_recorder()?;

    let app_metrics = async {
        let metrics_router = Router::new()
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .fallback(notfound_404);

        let router = metrics_router.layer(CatchPanicLayer::custom(runtime::catch_panic));

        // Spawn tick-driven process collection task
        tokio::task::spawn(process::collect_metrics(
            settings.monitoring().process_collector_interval,
        ));

        serve("Metrics", router, settings.server().metrics_port).await
    };

    let app = async {
        let req_id = HeaderName::from_static(REQUEST_ID);
        let router = router::setup_app_router()
            .route_layer(axum::middleware::from_fn(middleware::metrics::track))
            .layer(Extension(env))
            // Include trace context as header into the response.
            .layer(OtelInResponseLayer::default())
            // Opentelemetry tracing middleware.
            // This returns a `TraceLayer` configured to use
            // OpenTelemetry’s conventional span field names.
            .layer(OtelAxumLayer::default())
            // Set and propagate "request_id" (as a ulid) per request.
            .layer(
                ServiceBuilder::new()
                    .set_request_id(req_id.clone(), MakeRequestUlid)
                    .propagate_request_id(req_id),
            )
            // Applies the `tower_http::timeout::Timeout` middleware which
            // applies a timeout to requests.
            .layer(TimeoutLayer::new(Duration::from_millis(
                settings.server().timeout_ms,
            )))
            // Catches runtime panics and converts them into
            // `500 Internal Server` responses.
            .layer(CatchPanicLayer::custom(runtime::catch_panic))
            // Mark headers as sensitive on both requests and responses.
            .layer(SetSensitiveHeadersLayer::new(once(HeaderName::from_static(header::AUTHORIZATION.as_str()))))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

        serve("Application", router, settings.server().port).await
    };

    tokio::try_join!(app, app_metrics)?;
    Ok(())
}

async fn serve(name: &str, app: Router, port: u16) -> Result<()> {
    // let bind_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    info!(
        subject = "app_start",
        category = "init",
        "{} server listening on {}",
        name,
        port
    );

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown())
        .await
        .unwrap();

    Ok(())
}

/// Captures and waits for system signals.
async fn shutdown() {
    #[cfg(unix)]
    let term = async {
        signal(SignalKind::terminate())
            .expect("Failed to listen for SIGTERM")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    tokio::select! {
        _ = signal::ctrl_c() => {}
        _ = term => {}
    }
}

/// Setup all [tracing][tracing] layers for storage, request/response tracing,
/// logging and metrics.
fn setup_tracing(
    writer: tracing_appender::non_blocking::NonBlocking,
    settings_otel: &Otel,
) -> Result<()> {
    let tracer = init_tracer(settings_otel)?;

    let registry = tracing_subscriber::Registry::default()
        .with(StorageLayer.with_filter(LevelFilter::TRACE))
        .with(
            tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(LevelFilter::DEBUG)
                .with_filter(dynamic_filter_fn(|_metadata, ctx| {
                    !ctx.lookup_current()
                        // Exclude the rustls session "Connection" events
                        // which don't have a parent span
                        .map(|s| s.parent().is_none() && s.name() == "Connection")
                        .unwrap_or_default()
                })),
        )
        .with(LogFmtLayer::new(writer).with_target(true).with_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new(
                    std::env::var("RUST_LOG")
                        .unwrap_or_else(|_| "{{crate_name}}=info,tower_http=info,reqwest_retry=info,axum_tracing_opentelemetry=info".into()),
                )
            }),
        ))
        .with(
            MetricsLayer
                .with_filter(LevelFilter::TRACE)
                .with_filter(filter_fn(|metadata| {
                    // Filter and allow only:
                    // a) special metric prefix;
                    // b) any event
                    metadata.name().starts_with(METRIC_META_PREFIX) || metadata.is_event()
                })),
        );

    #[cfg(all(feature = "console", tokio_unstable))]
    #[cfg_attr(docsrs, doc(cfg(feature = "console")))]
    {
        let console_layer = console_subscriber::ConsoleLayer::builder()
            .retention(Duration::from_secs(60))
            .spawn();

        registry.with(console_layer).init();
    }

    #[cfg(any(not(feature = "console"), not(tokio_unstable)))]
    {
        registry.init();
    }

    Ok(())
}
