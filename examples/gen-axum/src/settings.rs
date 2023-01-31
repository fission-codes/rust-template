//! Settings / Configuration.

use config::{Config, ConfigError, Environment, File};
use http::Uri;
use serde::Deserialize;
use serde_with::serde_as;
use std::{path::PathBuf, time::Duration};

/// Names of environments for gen-axum.
/// Overrides serialization to force lower case in settings and
/// environment variables
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AppEnvironment {
    /// Local environment (local testing).
    Local,
    /// Official Develop environment.
    Dev,
    /// Official environment.
    Staging,
    /// Official Production environment.
    Prod,
}

/// Implement display to force environment to lower case
impl std::fmt::Display for AppEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

/// Server settings.
#[derive(Debug, Deserialize)]
pub struct Server {
    /// Server [AppEnvironment].
    pub environment: AppEnvironment,
    /// Server port.
    pub port: u16,
    /// Server metrics port.
    pub metrics_port: u16,
    /// Server timeout in milliseconds.
    pub timeout_ms: u64,
}

/// Process monitoring settings.
#[derive(Debug, Deserialize)]
pub struct Monitoring {
    /// Monitoring collection interval.
    pub process_collector_interval: u64,
}

/// [Opentelemetry] settings.
///
/// [Opentelemetry]: https://opentelemetry.io/
#[serde_as]
#[derive(Deserialize)]
pub struct Otel {
    /// Exporter [Uri] for OTEL protocol.
    #[serde(with = "http_serde::uri")]
    pub exporter_otlp_endpoint: Uri,
}

impl std::fmt::Debug for Otel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Otel")
            .field("exporter_otlp_endpoint", &self.exporter_otlp_endpoint)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
/// Application settings.
pub struct Settings {
    monitoring: Monitoring,
    server: Server,
    otel: Otel,
}

impl Settings {
    /// Environment settings getter.
    pub fn environment(&self) -> AppEnvironment {
        self.server().environment
    }

    /// Monitoring settings getter.
    pub fn monitoring(&self) -> &Monitoring {
        &self.monitoring
    }

    /// OTEL settings getter.
    pub fn otel(&self) -> &Otel {
        &self.otel
    }

    /// Server settings getter.
    pub fn server(&self) -> &Server {
        &self.server
    }
}

impl Settings {
    /// Load settings.
    pub fn load() -> Result<Self, ConfigError> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/settings.toml");
        // inject environment variables naming them properly on the settings
        // e.g. [database] url="foo"
        // would be injected with environment variable APP_DATABASE_URL="foo"
        // use one underscore as defined by the separator below
        let s = Config::builder()
            .add_source(File::with_name(&path.as_path().display().to_string()))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;
        s.try_deserialize()
    }
}

/// Http-client retry options.
#[derive(Clone, Debug, Deserialize)]
pub struct HttpClientRetryOptions {
    /// Retry count.
    pub count: u8,
    /// Retry lower bounds for [reqwest_retry::policies::ExponentialBackoff].
    pub bounds_low_ms: u64,
    /// Retry upper bounds for [reqwest_retry::policies::ExponentialBackoff].
    pub bounds_high_ms: u64,
}

impl Default for HttpClientRetryOptions {
    fn default() -> Self {
        Self {
            bounds_high_ms: 5_000,
            bounds_low_ms: 100,
            count: 3,
        }
    }
}

/// Settings for Http clients.
#[derive(Clone, Debug, Deserialize)]
pub struct HttpClient {
    /// Optional timeout for idle sockets being kept-alive.
    /// Using `None` to disable timeout.
    pub pool_idle_timeout_ms: Option<u64>,
    #[serde(default)]
    /// Http-client retry options.
    pub retry_options: HttpClientRetryOptions,
    /// Client timeout in milliseconds.
    pub timeout_ms: u64,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            pool_idle_timeout_ms: Some(5_000),
            retry_options: HttpClientRetryOptions::default(),
            timeout_ms: 30_000,
        }
    }
}

impl HttpClient {
    /// Convert `pool_idle_timeout_ms` to [Duration].
    pub fn pool_idle_timeout(&self) -> Option<Duration> {
        self.pool_idle_timeout_ms.and_then(|timeout| {
            if timeout != 0 {
                Some(Duration::from_millis(timeout))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub(crate) struct Client {
        #[serde(default)]
        http_client: HttpClient,
    }

    #[test]
    fn test_default_http_client_settings() {
        let settings = Client {
            http_client: HttpClient::default(),
        };

        assert_eq!(
            settings.http_client.pool_idle_timeout(),
            Some(Duration::from_millis(5_000))
        );
        assert_eq!(settings.http_client.retry_options.bounds_high_ms, 5_000);
        assert_eq!(settings.http_client.retry_options.bounds_low_ms, 100);
        assert_eq!(settings.http_client.retry_options.count, 3);
        assert_eq!(settings.http_client.timeout_ms, 30_000);
    }

    #[test]
    fn test_http_client_overrides() {
        let settings = Client {
            http_client: HttpClient {
                pool_idle_timeout_ms: Some(0),
                retry_options: HttpClientRetryOptions {
                    bounds_low_ms: 10,
                    bounds_high_ms: 100,
                    count: 10,
                },
                timeout_ms: 100,
            },
        };

        assert_eq!(settings.http_client.pool_idle_timeout(), None);
        assert_eq!(settings.http_client.retry_options.bounds_high_ms, 100);
        assert_eq!(settings.http_client.retry_options.bounds_low_ms, 10);
        assert_eq!(settings.http_client.retry_options.count, 10);
        assert_eq!(settings.http_client.timeout_ms, 100);
    }

    #[test]
    fn test_http_client_partial_overrides() {
        let settings = Client {
            http_client: HttpClient {
                pool_idle_timeout_ms: Some(5_000),
                retry_options: HttpClientRetryOptions {
                    bounds_low_ms: 10,
                    bounds_high_ms: 5_000,
                    count: 1,
                },
                timeout_ms: 10_000,
            },
        };

        assert_eq!(
            settings.http_client.pool_idle_timeout(),
            Some(Duration::from_millis(5_000))
        );
        assert_eq!(settings.http_client.retry_options.bounds_high_ms, 5_000);
        assert_eq!(settings.http_client.retry_options.bounds_low_ms, 10);
        assert_eq!(settings.http_client.retry_options.count, 1);
        assert_eq!(settings.http_client.timeout_ms, 10_000);
    }
}
