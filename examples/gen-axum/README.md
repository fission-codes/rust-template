<div align="center">
  <a href="https://github.com/fission-codes/gen-axum" target="_blank">
    <img src="https://raw.githubusercontent.com/fission-codes/gen-axum/main/assets/a_logo.png" alt="gen-axum Logo" width="100"></img>
  </a>

  <h1 align="center">gen-axum</h1>

  <p>
    <a href="https://crates.io/crates/gen-axum">
      <img src="https://img.shields.io/crates/v/gen-axum?label=crates" alt="Crate">
    </a>
    <a href="https://github.com/fission-codes/gen-axum/blob/main/LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="https://github.com/fission-codes/gen-axum/blob/main/LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>
    <a href="https://docs.rs/gen-axum">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

## Outline

- [Running the Webserver](#running-the-webserver)
- [Testing the Project](#testing-the-project)
- [Running gen-axum on Docker](#running-gen-axum-on-docker)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Running the Webserver

To start-up the [axum][axum] webserver, just run:

```console
cargo run
```

This will start-up the service, running on 2 ports:

* `3000`: main `gen-axum` application, including `/healthcheck`, etc.
* `4000`: `/metrics`

Upon running the application locally, [OpenAPI][openapi]
documentation is available as a [swagger-ui][swagger]
at `http://localhost:3000/swagger-ui/`. Read more in
[Docs and OpenAPI](#docs-and-openapi).

For local development with logs displayed using ANSI terminal colors, we
recommend running:

```console
cargo run --features ansi-logs
```

### Debugging and Diagnostics

To better help diagnose and debug your server application, you can run:

``` console
RUSTFLAGS="--cfg tokio_unstable" cargo run --features console, ansi-logs
```

This command uses a compile-time feature-flag, `console`, to give us local
access to [`tokio-console`][tokio-console], a diagnostics and debugging
tool for asynchronous Rust programs, akin to [pprof][pprof], `htop`/`top`,
etc. You can install `tokio-console` using `cargo`:

``` console
cargo install --locked tokio-console
```

Once executed, just run `tokio-console --retain-for <*>min` to use it and explore.

### Configuration

`gen-axum` contains a file for [configuration settings](./config/settings.toml),
loaded by the application when it starts. Configuration can be overridden using
environment variables that begin with an *APP* prefix. To allow for underscores
in variable names, use separators with two underscores between *APP* and the name
of the setting, for example:

```bash
export APP__SERVER__ENVIRONMENT="dev"
```

This export would override this setting in the [default config](./config/settings.toml):

```toml
[server]
environment = "local"
```

### Making HTTP Client Requests with [Reqwest][reqwest]

This web framework includes the [reqwest][reqwest] HTTP Client library for
making requests to external APIs and services, separate from the `axum` webserver
itself. We use the [reqwest-middleware][reqwest-middleware] crate for
wrapping around `reqwest` requests for client middleware chaining, giving us
metrics, retries, and tracing out of the box. We have an
[integration test](./gen-axum/tests/integration_test.rs),
which demonstrates how to build a client with middleware and configuration:

```rust
 // reqwest::Client by default has a timeout of 30s
 let reqwest_client = Client::builder()
     .pool_idle_timeout(settings.http_client.pool_idle_timeout())
     .timeout(Duration::from_millis(settings.http_client.timeout_ms))
     .build();

 Ok(Self {
     client: ClientBuilder::new(reqwest_client?)
         .with(TracingMiddleware::<ExtendedTrace>::new())
         .with(Logger)
         .with(RetryTransientMiddleware::new_with_policy(
             retry_policy,
             "AClient".to_string(),
         ))
         .with(Metrics {
             name: "AClient".to_string(),
         })
         .build(),

     url: settings.url.to_string(),
 })
```

*Note*: Our [logging middleware](./gen-axum/src/middleware/logging.rs)
implements traits for both `axum` and `reqwest` `Request` types. Additionally,
we implement an HTTP Client-specific middleware for deriving metrics for each
external, `reqwest` request in
[middleware/client.metrics.rs](./gen-axum/src/middleware/client/metrics.rs).
For the `axum` webserver itself, metrics are derived via
[middleware/metrics.rs](./gen-axum/src/middleware/metrics.rs).

## Testing the Project

- Run tests

  ```console
  cargo test
  ```

## Running gen-axum on Docker

We recommend setting your [Docker Engine][docker-engine] configuration
with `experimental` and `buildkit` set to `true`, for example:

``` json
{
  "builder": {
    "gc": {
      "defaultKeepStorage": "20GB",
      "enabled": true
    }
  },
  "experimental": true,
  "features": {
    "buildkit": true
  }
}
```

- Build a multi-plaform Docker image via [buildx][buildx]:

  ```console
  docker buildx build --platform=linux/amd64,linux/arm64 -t gen-axum --progress=plain .
  ```

- Run a Docker image (depending on your platform):

  ```console
  docker run --platform=linux/amd64 -t gen-axum
  ```

### Nix

This repository contains a [Nix flake][nix-flake] that initiates both the Rust
toolchain set in [rust-toolchain.toml](./rust-toolchain.toml) and a
[pre-commit hook](#pre-commit-hook). It also installs helpful cargo binaries for
development. Please install [nix][nix] and [direnv][direnv] to get started.

Run `nix develop` or `direnv allow` to load the `devShell` flake output,
according to your preference.

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend by default.

### Pre-commit Hook

This project recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Docs and OpenAPI

If you make any changes to [axum][axum] routes/handlers, make sure to add/update
OpenAPI specifications. You can run `cargo run --bin openapi`
to generate an updated specification .json file, located
[here](./docs/specs/latest.json).

An example of adding an OpenAPI specification is the following:

```rust
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Ping successful"),
        (status = 500, description = "Ping not successful", body=AppError)
    )
)]
pub async fn get() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
  ```

Of note, once you add the [*utoipa*][utoipa] attribute macro to a route,
you should also update the `ApiDoc` struct in [*src/docs.rs*](./src/docs.rs):

``` rust
/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(health::healthcheck, ping::get),
        components(schemas(AppError)),
        tags(
            (name = "", description = "gen-axum service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
```

### Recording: Logging, Tracing, and Metrics Layers

For logs, traces, and metrics, `gen-axum` utilizes several log levels
and middleware trace layers to control how events are recorded. The trace layers
include a: (1) *storage layer*; (2) *otel layer*; (3) *format layer*; and a
(4) *metrics layer*. The log levels include: (1) _trace_; (2) _debug_;
(3) _info_; (4) _warn_; and (5) _error_. All of this leverages the
[tracing][tracing] library and it's related extensions. This approach
is heavily inspired by [*Composing an observable Rust application*][composing-rust].

At its core, the [storage layer](./src/tracing_layers/storage_layer.rs) exists
to capture everything flowing through `gen-axum` before events are
diffracted to their respective log levels--this way it is possible for
`gen-axum` to maintain contextual trace information throughout the
lifetime of any event, no matter the log level.

The final layer is the [metrics layer](./src/tracing_layers/metrics_layer.rs),
which, **of note**, removes the stored span information upon span closure.

#### How Does Logging Work?

The [logging middleware](./src/middleware/logging.rs) automatically drives
request/response logging, taking into account status codes and helpful
contextual information.

For logging, we use the [tracing][tracing-log] library and structure logs in
[`logfmt`][logfmt] style. The implementation of the log generation is inspired
by [influxdata's (Influx DB's) version][influx-logfmt].
When defining log functions for output, please define them like so:

```rust
self.healthcheck()
    .await
    .map(|_| {
        info!(
            subject = "postgres",
            category = "db",
            "connection to PostgresDB successful"
        )
    })
    .map_err(|e| {
        error!(
            subject = "postgres",
            category = "db",
            error=?e,
            "failed to connect to PostgresDB",
        );
```

#### How Does Tracing work?

`gen-axum` implements hooks around the creation and closing of spans
across the lifetime of events and requests in order to track the entire *trace*
of that event or request. Each created span has a unique span id that will match
its close. Below is an example which demonstrates the opening of span with an id
of `2251799813685249`, then a logging event which occurs within that span, and
then closing of that span once it's complete.

```console
level=INFO span_name="HTTP request" span=2251799813685249 span_event=new_span timestamp=2023-01-29T15:06:42.188395Z http.method=GET http.client_ip=127.0.0.1:59965 http.host=localhost:3000 trace_id=fa9754fa3142db2c100a8c47f6dd391d http.route=/ping
level=INFO subject=request category=http.request msg="started processing request" request_path=/ping authorization=null target="project::middleware::logging" location="project/src/middleware/logging.rs:123" timestamp=2023-01-29T15:06:42.188933Z span=2251799813685249 otel.name="GET /ping" http.method=GET http.scheme=HTTP http.client_ip=127.0.0.1:59965 http.flavor=1.1 otel.kind=server http.user_agent=curl/7.85.0 http.host=localhost:3000 trace_id=fa9754fa3142db2c100a8c47f6dd391d http.target=/ping http.route=/ping
level=INFO span_name="HTTP request" span=2251799813685249 span_event=close_span timestamp=2023-01-29T15:06:42.192221Z http.method=GET latency_ms=3 http.client_ip=127.0.0.1:59965 http.host=localhost:3000 trace_id=fa9754fa3142db2c100a8c47f6dd391d http.route=/ping
```

#### How Does Instrumentation Work?

When leveraging [tracing's][tracing] [instrument][tracing-instr] functionality,
we can instrument a function to create and enter a tracing span every time that
function is called. There are two ways to use instrumentation:

* instrumentation *macros*
* instrumentation *methods*.

#### Instrumentation Macros (used above fn signatures)

```rust
#[instrument(
    level = "info",
    name = "gen-axum.songs.handler.POST",
    skip_all,
    fields(category = "http.handler", subject = "songs")
)]
pub async fn post(db: Extension<PG>,...)
```

#### Instrumentation Method (used with async closures, follows_from reference)

```rust
// Start a span around the context process spawn
let process_span = debug_span!(
    parent: None,
    "process.async",
    subject = "songs.async",
    category = "songs"
);
process_span.follows_from(Span::current());

tokio::spawn(
    async move {
        match context.process().await {
            Ok(r) => debug!(event=?r, "successfully processed song addition"),
            Err(e) => warn!(error=?e, "failed processing song"),
        }
    }
    .instrument(process_span),
);
```

#### Deriving Metrics through Instrumentation

If a function is instrumented with a special `.record` prefix in the `name`
field, then, as part of the its execution, a `counter` will
automatically be incremented and a `histogram` recorded for that
function's span context (start-to-end):

```rust
#[instrument(
    level = "info",
    name = "record.save_event",
    skip_all,
    fields(category="db", subject="postgres", event_id = %event.event_id,
           event_type=%event.event_type,
           metric_name="db_event",
           metric_label_event_type=%event.event_type
    )
    err(Display)
)]
async fn save_event(...) -> ... {
```

These metrics are derived via the [metrics layer](./src/tracing_layers/metrics_layer.rs)
where the metrics are stripped off the `.record` prefix and then recorded with
the [metrics-rs][metrics-rs] library:

```rust
let span_name = span
    .name()
    .strip_prefix(METRIC_META_PREFIX)
    .unwrap_or_else(|| span.name());
...
...
metrics::increment_counter!(format!("{name}_total"), &labels);
metrics::histogram!(
    format!("{name}_duration_seconds"),
    elapsed_secs_f64,
    &labels
);
```

#### How is [OTEL][otel] incorporated for exporting (distributed) tracing information?

The [axum-tracing-opentelemetry][axum-otel] crate provides middleware for adding
OTEL integration to a `tower` service, an extended [`Tracelayer`][tower-tracelayer],
setting OTEL span information when `gen-axum` application
[routes](./src/routes) are executed.

OTEL trace information is exported using a [opentelemetry propagation layer][otel-layer],
which is registered along with the other layers, for example storage, logging, metrics.
This information is exported over [grpc][grpc], using a Rust implementation of
the [opentelemetry otlp][otel-otlp] specification, codified in our
[tracer module](./src/tracer.rs). With the proper settings and setup, this will
work for local development, exporting to a service like [Jaeger][jaeger] or for
sending traces to [Honeycomb][honeycomb] or a similar cloud service.

### Recommended Development Flow

- We recommend leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.
- We recommend using [cargo-udeps][cargo-udeps] for removing unused dependencies
  before commits and pull-requests.

### Conventional Commits

This project *lightly* follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help

For usage questions, usecases, or issues please open an issue in our repository.

We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache])
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT][mit])

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


[apache]: https://www.apache.org/licenses/LICENSE-2.0
[axum]: https://docs.rs/axum/latest/axum/
[axum-otel]: https://github.com/davidB/axum-tracing-opentelemetry
[buildx]: https://github.com/docker/buildx
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[composing-rust]: https://blog.logrocket.com/composing-underpinnings-observable-rust-application/
[config-rs]: https://github.com/mehcode/config-rs
[docker-engine]: https://docs.docker.com/engine/
[direnv]:https://direnv.net/
[honeycomb]: https://www.honeycomb.io/
[influx-logfmt]: https://github.com/influxdata/influxdb_iox/tree/main/logfmt
[irust]: https://github.com/sigmaSd/IRust
[jaeger]: https://www.jaegertracing.io/
[logfmt]: https://brandur.org/logfmt
[mit]: http://opensource.org/licenses/MIT
[nix]:https://nixos.org/download.html
[nix-flake]: https://nixos.wiki/wiki/Flakes
[openapi]: https://swagger.io/specification/
[otel]: https://opentelemetry.io/docs/
[otel-issue]: https://github.com/open-telemetry/opentelemetry-rust/issues/934
[otel-layer]: https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/struct.OpenTelemetryLayer.html
[otel-otlp]: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/otlp.md
[pprof]: https://github.com/google/pprof
[pre-commit]: https://pre-commit.com/
[reqwest]: https://github.com/seanmonstar/reqwest
[reqwest-middleware]: https://github.com/TrueLayer/reqwest-middleware
[swagger]: https://swagger.io/tools/swagger-ui/
[tokio-console]: https://tokio-console.netlify.app/console_subscriber/
[tower-tracelayer]: https://docs.rs/tower-http/latest/tower_http/trace/struct.TraceLayer.html
[tracing]: https://github.com/tokio-rs/tracing
[tracing-instr]: https://docs.rs/tracing-attributes/latest/tracing_attributes/attr.instrument.html
[utoipa]: https://github.com/juhaku/utoipa
