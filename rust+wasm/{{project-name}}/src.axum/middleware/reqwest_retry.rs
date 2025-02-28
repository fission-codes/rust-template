//! [RetryTransientMiddleware] implements retrying requests on transient errors.
//! This variant minorly extends [TrueLayer's request-retry middleware].
//!
//! [TrueLayer's request-retry middleware]:
//! <https://github.com/TrueLayer/reqwest-middleware/blob/main/reqwest-retry/src/middleware.rs>

use crate::middleware::client;
use anyhow::anyhow;
use chrono::Utc;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Error, Middleware, Next, Result};
use reqwest_retry::{RetryPolicy, Retryable};
use retry_policies::RetryDecision;
use std::time::SystemTime;
use tracing::warn;

/// We limit the number of retries to a maximum of `10` to avoid stack-overflow issues due to the recursion.
static MAXIMUM_NUMBER_OF_RETRIES: u32 = 10;

/// `RetryTransientMiddleware` offers retry logic for requests that fail in a transient manner
/// and can be safely executed again.
///
/// Currently, it allows setting a [RetryPolicy][retry_policies::RetryPolicy] algorithm for calculating the __wait_time__
/// between each request retry.
///
///```rust,no_run
///     use {{crate_name}}::middleware::reqwest_retry::RetryTransientMiddleware;
///     use reqwest_middleware::ClientBuilder;
///
///     use reqwest_retry::policies::ExponentialBackoff;
///     use reqwest::Client;
///
///     // We create a ExponentialBackoff retry policy which implements `RetryPolicy`.
///     let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
///
///     let client_name = "YoMTVDocs";
///     let retry_transient_middleware = RetryTransientMiddleware::new_with_policy(retry_policy, client_name.to_string());
///     let reqwest_client = Client::builder()
///         .pool_idle_timeout(std::time::Duration::from_millis(50))
///         .timeout(std::time::Duration::from_millis(100))
///         .build()?;
///     let client = ClientBuilder::new(reqwest_client).with(retry_transient_middleware).build();
///     # Ok::<(), reqwest::Error>(())
///```
///
/// # Note
///
/// This middleware always errors when given requests with streaming bodies, before even executing
/// the request. When this happens you'll get an [`Error::Middleware`] with the message
/// 'Request object is not clonable. Are you passing a streaming body?'.
///
/// Some workaround suggestions:
/// * If you can fit the data in memory, you can instead build static request bodies e.g. with
///   `Body`'s `From<String>` or `From<Bytes>` implementations.
/// * You can wrap this middleware in a custom one which skips retries for streaming requests.
/// * You can write a custom retry middleware that builds new streaming requests from the data
///   source directly, avoiding the issue of streaming requests not being clonable.
#[derive(Debug)]
pub struct RetryTransientMiddleware<T: RetryPolicy + Send + Sync + 'static> {
    client_name: String,
    retry_policy: T,
}

impl<T: RetryPolicy + Send + Sync> RetryTransientMiddleware<T> {
    /// Construct `RetryTransientMiddleware` with  a [retry_policy][retry_policies::RetryPolicy].
    pub fn new_with_policy(retry_policy: T, client_name: String) -> Self {
        Self {
            client_name,
            retry_policy,
        }
    }
}

#[async_trait::async_trait]
impl<T: RetryPolicy + Send + Sync> Middleware for RetryTransientMiddleware<T> {
    async fn handle(
        &self,
        request: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // TODO: Ideally we should create a new instance of the `Extensions` map to pass
        // downstream. This will guard against previous retries poluting `Extensions`.
        // That is, we only return what's populated in the typemap for the last retry attempt
        // and copy those into the the `global` Extensions map.
        self.execute_with_retry(request, next, extensions).await
    }
}

impl<T: RetryPolicy + Send + Sync> RetryTransientMiddleware<T> {
    /// This function will try to execute the request, if it fails
    /// with an error classified as transient it will call itself
    /// to retry the request.
    async fn execute_with_retry<'a>(
        &'a self,
        request: Request,
        next: Next<'a>,
        extensions: &'a mut Extensions,
    ) -> Result<Response> {
        let mut n_past_retries = 0;
        let start_time = SystemTime::now();

        loop {
            // Cloning the request object before-the-fact is not ideal..
            // However, if the body of the request is not static, e.g of type `Bytes`,
            // the Clone operation should be of constant complexity and not O(N)
            // since the byte abstraction is a shared pointer over a buffer.
            let duplicate_request = request.try_clone().ok_or_else(|| {
                Error::Middleware(anyhow!(
                    "Request object is not clonable. Are you passing a streaming body?".to_string()
                ))
            })?;

            // Only generate metrics here upon retries, i.e. after
            // `n_past_retries`==0, 0 being the init index
            let result = if n_past_retries > 0 {
                self.handle_retry_metric(duplicate_request, extensions, next.clone())
                    .await
            } else {
                next.clone().run(duplicate_request, extensions).await
            };

            // We classify the response which will return None if not
            // errors were returned.
            break match Retryable::from_reqwest_response(&result) {
                Some(retryable)
                    if retryable == Retryable::Transient
                        && n_past_retries < MAXIMUM_NUMBER_OF_RETRIES =>
                {
                    // If the response failed and the error type was transient
                    // we can safely try to retry the request.
                    let retry_decicion = self.retry_policy.should_retry(start_time, n_past_retries);
                    if let RetryDecision::Retry { execute_after } = retry_decicion {
                        let duration = (chrono::DateTime::from(execute_after) - Utc::now())
                            .to_std()
                            .map_err(Error::middleware)?;
                        warn!(
                            subject = "client.retry",
                            category = "client",
                            retry_attempt = n_past_retries + 1,
                            wait_duration = ?duration,
                            "retrying call with backoff policy",
                        );
                        // Sleep the requested amount before we try again.
                        tokio::time::sleep(duration).await;

                        n_past_retries += 1;
                        continue;
                    } else {
                        result
                    }
                }
                Some(_) | None => result,
            };
        }
    }

    /// Handle response metrics associated with a retry in the loop.
    async fn handle_retry_metric<'a>(
        &'a self,
        request: Request,
        extensions: &mut Extensions,
        next: Next<'a>,
    ) -> Result<Response> {
        let request_path = request.url().path().to_string();
        let method = request.method().clone();

        let result = next.run(request, extensions).await;

        let labels = vec![
            ("client", self.client_name.to_string()),
            ("method", method.to_string()),
            ("request_path", request_path.clone()),
        ];

        let extended_labels = client::metrics::extend_labels_for_response(labels, &result);

        metrics::counter!("client_http_requests_retry_total").increment(1);
        metrics::counter!("client_http_requests_retry_total", "request_path" => request_path)
            .increment(1);
        metrics::counter!("client_http_requests_retry_total", &extended_labels).increment(1);
        result
    }
}
