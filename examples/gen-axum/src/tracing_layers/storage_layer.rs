//! Storage layer.

use std::{borrow::Cow, collections::HashMap, fmt, time::Instant};
use tracing::{
    field::{Field, Visit},
    span::{Attributes, Record},
    Event, Id, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// Storage fields for events.
const ON_EVENT_KEEP_FIELDS: [&str; 1] = ["error"];

const TRACE_ID: &str = "trace_id";
const PARENT_SPAN: &str = "parent_span";
const FOLLOWS_FROM_TRACE_ID: &str = "follows_from.trace_id";
const FOLLOWS_FROM_FIELD: &str = "follows_from";
const LATENCY_FIELD: &str = "latency_ms";

/// Storage layer for contextual trace information.
///
/// Prepend to custom [LogFmtLayer](crate::tracing_layers::format_layer::LogFmtLayer).
#[derive(Clone, Debug)]
pub struct StorageLayer;

#[derive(Clone, Debug, Default)]
pub(crate) struct Storage<'a> {
    values: HashMap<&'a str, Cow<'a, str>>,
}

impl<'a> Storage<'a> {
    pub(crate) fn values(&self) -> &HashMap<&'a str, Cow<'a, str>> {
        &self.values
    }
}

impl Visit for Storage<'_> {
    /// Visit a signed 64-bit integer value.
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.values
            .insert(field.name(), Cow::from(value.to_string()));
    }

    /// Visit an unsigned 64-bit integer value.
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.values
            .insert(field.name(), Cow::from(value.to_string()));
    }

    /// Visit a boolean value.
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.values
            .insert(field.name(), Cow::from(value.to_string()));
    }

    /// Visit a string value.
    fn record_str(&mut self, field: &Field, value: &str) {
        self.values
            .insert(field.name(), Cow::from(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        // Note this appears to be invoked via `debug!` and `info! macros
        match field.name() {
            name if name.starts_with("log.") => (),
            _ => {
                let debug_formatted = format!("{value:?}");
                self.values.insert(field.name(), Cow::from(debug_formatted));
            }
        }
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        match field.name() {
            name if name.starts_with("log.") => (),
            _ => {
                let display_formatted = format!("{value}");
                self.values
                    .insert(field.name(), Cow::from(display_formatted));
            }
        }
    }
}

impl<S> Layer<S> for StorageLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found");

        // We want to inherit the fields from the parent span, if there is one.
        let mut visitor = if let Some(parent_span) = span.parent() {
            let mut extensions = parent_span.extensions_mut();

            let mut inner = extensions
                .get_mut::<Storage<'_>>()
                .map(|v| v.to_owned())
                .unwrap_or_default();

            inner.values.insert(
                PARENT_SPAN,
                Cow::from(parent_span.id().into_u64().to_string()),
            );
            inner
        } else {
            Storage::default()
        };

        let mut extensions = span.extensions_mut();

        attrs.record(&mut visitor);
        extensions.insert(visitor);
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found");

        let mut extensions = span.extensions_mut();
        let visitor = extensions
            .get_mut::<Storage<'_>>()
            .expect("Visitor not found on 'record'!");

        values.record(visitor);
    }

    fn on_follows_from(&self, span: &Id, follows: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found");
        let follows_span = ctx.span(follows).expect("Span not found");

        let mut extensions = span.extensions_mut();
        let follows_extensions = follows_span.extensions();

        if let Some((visitor, follows_visitor)) = extensions
            .get_mut::<Storage<'_>>()
            .zip(follows_extensions.get::<Storage<'_>>())
        {
            // insert "follows_from" span name
            visitor
                .values
                .insert(FOLLOWS_FROM_FIELD, Cow::from(follows_span.name()));

            // insert "follows_from" trace_id
            let follows_trace = follows_visitor
                .values
                .get(TRACE_ID)
                .unwrap_or(&Cow::from(format!("{follows:?}")))
                .to_string();
            visitor
                .values
                .insert(FOLLOWS_FROM_TRACE_ID, Cow::from(follows_trace));
        };
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        ctx.lookup_current().map(|current_span| {
            let mut extensions = current_span.extensions_mut();
            extensions.get_mut::<Storage<'_>>().map(|visitor| {
                if event
                    .fields()
                    .any(|f| ON_EVENT_KEEP_FIELDS.contains(&f.name()))
                {
                    event.record(visitor);
                }
            })
        });
    }

    fn on_enter(&self, span: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found");

        let mut extensions = span.extensions_mut();
        if extensions.get_mut::<Instant>().is_none() {
            extensions.insert(Instant::now());
        }
    }
    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found");

        let mut extensions = span.extensions_mut();

        let elapsed_milliseconds = extensions
            .get_mut::<Instant>()
            .map(|i| i.elapsed().as_millis())
            .unwrap_or(0);

        let visitor = extensions
            .get_mut::<Storage<'_>>()
            .expect("Visitor not found on 'record'");

        visitor
            .values
            .insert(LATENCY_FIELD, Cow::from(format!("{elapsed_milliseconds}")));
    }
}
