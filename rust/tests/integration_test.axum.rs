use http::Uri;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_tracing::TracingMiddleware;
use serde::Deserialize;
use serde_with::serde_as;
use std::time::Duration;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};
use {{crate_name}}::{
    middleware::{
        client::metrics::Metrics, logging::Logger, reqwest_retry::RetryTransientMiddleware,
        reqwest_tracing::ExtendedTrace,
    },
    settings::{AppEnvironment, HttpClient, HttpClientRetryOptions, Settings},
};

/// Test loading settings.
#[test]
fn test_settings() {
    let settings = Settings::load().unwrap();
    assert_eq!(settings.environment(), AppEnvironment::Local);
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct ClientSettings {
    #[serde(default)]
    pub http_client: HttpClient,
    #[serde(with = "http_serde::uri")]
    pub url: Uri,
}

/// A reqwest-based HTTP client for all Sentilink API operations.
///
/// 500s are retried.
#[derive(Debug)]
struct AClient {
    client: ClientWithMiddleware,
    url: String,
}

impl AClient {
    fn load(settings: ClientSettings) -> anyhow::Result<Self> {
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                Duration::from_millis(settings.http_client.retry_options.bounds_low_ms),
                Duration::from_millis(settings.http_client.retry_options.bounds_high_ms),
            )
            .build_with_max_retries(settings.http_client.retry_options.count.into());

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
    }

    async fn query(&self) -> anyhow::Result<reqwest::Response> {
        // Send the actual http request.
        let response = self
            .client
            .get(format!("{}query", self.url.to_owned()))
            .send()
            .await?;
        Ok(response)
    }
}

/// Test example reqwest-client call via wiremock.
#[tokio::test]
async fn test_client() {
    let mock_server = MockServer::start().await;

    let settings = ClientSettings {
        http_client: HttpClient {
            pool_idle_timeout_ms: Some(5000),
            retry_options: HttpClientRetryOptions {
                bounds_low_ms: 100,
                bounds_high_ms: 5000,
                count: 3,
            },
            timeout_ms: 100,
        },
        url: mock_server.uri().parse::<Uri>().unwrap(),
    };

    let client = AClient::load(settings).unwrap();

    Mock::given(method("GET"))
        .and(path("/query"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1) // number of expected requests
        .mount(&mock_server)
        .await;

    let res = client.query().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
}
