//! Metrics layer.

use crate::tracing_layers::storage_layer::Storage;
use std::{borrow::Cow, time::Instant};
use tracing::{Id, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

const PREFIX_LABEL: &str = "metric_label_";
const METRIC_NAME: &str = "metric_name";
const OK: &str = "ok";
const ERROR: &str = "error";
const LABEL: &str = "label";
const RESULT_LABEL: &str = "result";
const SPAN_LABEL: &str = "span_name";

/// Prefix used for capturing metric spans/instrumentations.
pub const METRIC_META_PREFIX: &str = "record.";

/// Metrics layer for automatically deriving metrics for record.* events.
///
/// Append to custom [LogFmtLayer](crate::tracing_layers::format_layer::LogFmtLayer).
#[derive(Debug)]
pub struct MetricsLayer;

impl<S> Layer<S> for MetricsLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found");
        let mut extensions = span.extensions_mut();

        let elapsed_secs_f64 = extensions
            .get_mut::<Instant>()
            .map(|i| i.elapsed().as_secs_f64())
            .unwrap_or(0.0);

        if let Some(visitor) = extensions.get_mut::<Storage<'_>>() {
            let mut labels = vec![];
            for (key, value) in visitor.values() {
                if key.starts_with(PREFIX_LABEL) {
                    labels.push((
                        key.strip_prefix(PREFIX_LABEL).unwrap_or(LABEL),
                        value.to_string(),
                    ))
                }
            }

            let span_name = span
                .name()
                .strip_prefix(METRIC_META_PREFIX)
                .unwrap_or_else(|| span.name());

            labels.push((SPAN_LABEL, span_name.to_string()));

            let name = visitor
                .values()
                .get(METRIC_NAME)
                .unwrap_or(&Cow::from(span_name))
                .to_string();

            if visitor.values().contains_key(ERROR) {
                labels.push((RESULT_LABEL, String::from(ERROR)))
            } else {
                labels.push((RESULT_LABEL, String::from(OK)))
            }

            // Need to sort labels to remain the same across all metrics.
            labels.sort_unstable();

            metrics::increment_counter!(format!("{name}_total"), &labels);
            metrics::histogram!(
                format!("{name}_duration_seconds"),
                elapsed_secs_f64,
                &labels
            );

            // Remove storage as this is the last layer.
            extensions
                .remove::<Storage<'_>>()
                .expect("Visitor not found on 'close'");
        }
    }
}
