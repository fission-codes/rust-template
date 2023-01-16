//! [Logfmt]'ed event logging [Layer] with augmented telemetry info.
//!
//! Inspired by [influxdata's (Influx DB's) version].
//!
//! [Logfmt]: <https://brandur.org/logfmt>
//! [Layer]: tracing_subscriber::Layer
//! [influxdata's (Influx DB's) version]: <https://github.com/influxdata/influxdb_iox/tree/main/logfmt>

use crate::tracing_layers::storage_layer::Storage;
use parking_lot::RwLock;
use std::{
    borrow::Cow,
    fmt,
    io::{self, Write},
    time::SystemTime,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tracing::{
    field::{Field, Visit},
    metadata::LevelFilter,
    span::{Attributes, Id},
    Event, Level, Subscriber,
};
use tracing_subscriber::{fmt::MakeWriter, layer::Context, registry::LookupSpan, Layer};

/// Fields to persist from [Storage](Storage) for `new_span` logs via context.
const SPAN_FIELDS: [&str; 13] = [
    "category",
    "follows_from",
    "follows_from.trace_id",
    "http.client_ip",
    "http.host",
    "http.method",
    "http.route",
    "latency_ms",
    "parent_span",
    "request_id",
    "span",
    "subject",
    "trace_id",
];

/// Fields to skip from [Storage](Storage) spans for `on_event` logs via
/// context.
const ON_EVENT_SKIP_FIELDS: [&str; 6] = [
    "authorization",
    "category",
    "error",
    "msg",
    "return",
    "subject",
];

/// Fields to persist from [Storage](Storage) for `on_close` span logs via
/// context.
const ON_CLOSE_FIELDS: [&str; 12] = [
    "category",
    "follows_from",
    "follows_from.trace_id",
    "http.client_ip",
    "http.host",
    "http.method",
    "http.route",
    "latency_ms",
    "parent_span",
    "request_id",
    "subject",
    "trace_id",
];

#[cfg(feature = "ansi-logs")]
const GRAY: u8 = 245;

/// Logging layer for formatting and outputting event-driven logs.
#[derive(Debug)]
pub struct LogFmtLayer<Wr, W = fn() -> io::Stdout>
where
    Wr: Write,
    W: for<'writer> MakeWriter<'writer>,
{
    writer: W,
    printer: RwLock<FieldPrinter<Wr>>,
}

impl<Wr, W> LogFmtLayer<Wr, W>
where
    Wr: Write,
    W: for<'writer> MakeWriter<'writer, Writer = Wr>,
{
    /// Create a new logfmt Layer to pass into tracing_subscriber
    ///
    /// Note this layer simply formats and writes to the specified writer. It
    /// does not do any filtering for levels itself. Filtering can be done
    /// using a EnvFilter.
    ///
    /// For example:
    /// ```
    ///  use {{crate_name}}::tracing_layers::format_layer::LogFmtLayer;
    ///  use tracing_subscriber::{EnvFilter, prelude::*, self};
    ///
    ///  // setup debug logging level
    ///  std::env::set_var("RUST_LOG", "debug");
    ///
    ///  // setup formatter to write to stderr
    ///  let formatter =
    ///    LogFmtLayer::new(std::io::stderr);
    ///
    ///  tracing_subscriber::registry()
    ///    .with(EnvFilter::from_default_env())
    ///    .with(formatter)
    ///    .init();
    /// ```
    pub fn new(writer: W) -> Self {
        let make_writer = writer.make_writer();
        Self {
            writer,
            printer: RwLock::new(FieldPrinter::new(make_writer, true)),
        }
    }

    /// Control whether target and location attributes are displayed (on by default).
    ///
    /// Note: this API mimics that of other fmt layers in tracing-subscriber crate.
    pub fn with_target(self, display_target: bool) -> Self {
        let make_writer = self.writer.make_writer();
        Self {
            writer: self.writer,
            printer: RwLock::new(FieldPrinter::new(make_writer, display_target)),
        }
    }
}

impl<S, Wr, W> Layer<S> for LogFmtLayer<Wr, W>
where
    Wr: Write + 'static,
    W: for<'writer> MakeWriter<'writer> + 'static,
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn max_level_hint(&self) -> Option<LevelFilter> {
        None
    }

    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut p = self.printer.write();

        let metadata = ctx.metadata(id).expect("Span missing metadata");
        p.write_level(metadata.level());
        p.write_span_name(metadata.name());
        p.write_span_id(id);
        p.write_span_event("new_span");
        p.write_timestamp();

        let span = ctx.span(id).expect("Span not found");
        let extensions = span.extensions();
        if let Some(visitor) = extensions.get::<Storage<'_>>() {
            for (key, value) in visitor.values() {
                match *metadata.level() {
                    Level::TRACE | Level::DEBUG => p.write_kv(
                        decorate_field_name(translate_field_name(key)),
                        value.to_string(),
                    ),

                    _ => {
                        if SPAN_FIELDS.contains(key) {
                            p.write_kv(
                                decorate_field_name(translate_field_name(key)),
                                value.to_string(),
                            )
                        }
                    }
                }
            }
        }
        p.write_newline();
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut p = self.printer.write();

        p.write_level(event.metadata().level());
        event.record(&mut *p);

        //record source information
        p.write_source_info(event);
        p.write_timestamp();

        ctx.lookup_current().map(|current_span| {
            p.write_span_id(&current_span.id());
            let extensions = current_span.extensions();
            extensions.get::<Storage<'_>>().map(|visitor| {
                for (key, value) in visitor.values() {
                    if !ON_EVENT_SKIP_FIELDS.contains(key) {
                        p.write_kv(
                            decorate_field_name(translate_field_name(key)),
                            value.to_string(),
                        )
                    }
                }
            })
        });

        p.write_newline();
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let mut p = self.printer.write();

        let metadata = ctx.metadata(&id).expect("Span missing metadata");
        let span = ctx.span(&id).expect("Span not found");

        p.write_level(metadata.level());
        p.write_span_name(metadata.name());
        p.write_span_id(&span.id());
        p.write_span_event("close_span");
        p.write_timestamp();

        let mut extensions = span.extensions_mut();

        if let Some(visitor) = extensions.get_mut::<Storage<'_>>() {
            for (key, value) in visitor.values() {
                if ON_CLOSE_FIELDS.contains(key) {
                    p.write_kv(
                        decorate_field_name(translate_field_name(key)),
                        value.to_string(),
                    )
                }
            }
        }

        p.write_newline();
    }
}

/// This is responsible for actually printing log information to
/// the layer's writer.
#[derive(Debug)]
struct FieldPrinter<Wr: io::Write> {
    writer: Wr,
    display_target: bool,
}

impl<W: Write> FieldPrinter<W> {
    fn new(writer: W, display_target: bool) -> Self {
        Self {
            writer,
            display_target,
        }
    }

    #[cfg(feature = "ansi-logs")]
    fn write_level(&mut self, level: &Level) {
        let level_str = match *level {
            Level::TRACE => "trace",
            Level::DEBUG => "debug",
            Level::INFO => "info",
            Level::WARN => "warn",
            Level::ERROR => "error",
        }
        .to_uppercase();

        let level_name = match *level {
            Level::TRACE => ansi_term::Color::Purple,
            Level::DEBUG => ansi_term::Color::Blue,
            Level::INFO => ansi_term::Color::Green,
            Level::WARN => ansi_term::Color::Yellow,
            Level::ERROR => ansi_term::Color::Red,
        }
        .bold()
        .paint(level_str);

        write!(
            self.writer,
            r#"{}={}"#,
            decorate_field_name("level"),
            level_name
        )
        .ok();
    }

    #[cfg(not(feature = "ansi-logs"))]
    fn write_level(&mut self, level: &Level) {
        let level_str = match *level {
            Level::TRACE => "trace",
            Level::DEBUG => "debug",
            Level::INFO => "info",
            Level::WARN => "warn",
            Level::ERROR => "error",
        };

        write!(
            self.writer,
            r#"{}={}"#,
            decorate_field_name("level"),
            level_str
        )
        .ok();
    }

    fn write_span_name(&mut self, value: &str) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name("span_name"),
            quote_and_escape(value)
        )
        .ok();
    }

    fn write_source_info(&mut self, event: &Event<'_>) {
        if !self.display_target {
            return;
        }

        let metadata = event.metadata();

        if metadata.target() != "log" {
            write!(
                self.writer,
                " {}=\"{}\"",
                decorate_field_name("target"),
                quote_and_escape(metadata.target())
            )
            .ok();
        }

        if let Some(module_path) = metadata.module_path() {
            if metadata.target() != module_path {
                write!(
                    self.writer,
                    " {}=\"{}\"",
                    decorate_field_name("module_path"),
                    module_path
                )
                .ok();
            }
        }
        if let (Some(file), Some(line)) = (metadata.file(), metadata.line()) {
            write!(
                self.writer,
                " {}=\"{}:{}\"",
                decorate_field_name("location"),
                file,
                line
            )
            .ok();
        }
    }

    fn write_span_id(&mut self, id: &Id) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name("span"),
            id.into_u64()
        )
        .ok();
    }

    fn write_span_event(&mut self, hook: &str) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name("span_event"),
            hook
        )
        .ok();
    }

    fn write_timestamp(&mut self) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name("timestamp"),
            to_rfc3339(&SystemTime::now())
        )
        .ok();
    }

    fn write_kv(&mut self, key: String, value: String) {
        write!(self.writer, " {}={}", key, quote_and_escape(value.as_str())).ok();
    }

    fn write_newline(&mut self) {
        writeln!(self.writer).ok();
    }
}

impl<W: io::Write> Visit for FieldPrinter<W> {
    /// Visit a signed 64-bit integer value.
    fn record_i64(&mut self, field: &Field, value: i64) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name(translate_field_name(field.name())),
            value
        )
        .ok();
    }

    /// Visit an unsigned 64-bit integer value.
    fn record_u64(&mut self, field: &Field, value: u64) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name(translate_field_name(field.name())),
            value
        )
        .ok();
    }

    /// Visit a boolean value.
    fn record_bool(&mut self, field: &Field, value: bool) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name(translate_field_name(field.name())),
            value
        )
        .ok();
    }

    /// Visit a string value.
    fn record_str(&mut self, field: &Field, value: &str) {
        write!(
            self.writer,
            " {}={}",
            decorate_field_name(translate_field_name(field.name())),
            quote_and_escape(value)
        )
        .ok();
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        // Note this appears to be invoked via `debug!` and `info! macros
        let formatted_value = format!("{value:?}");
        write!(
            self.writer,
            " {}={}",
            decorate_field_name(translate_field_name(field.name())),
            quote_and_escape(&formatted_value)
        )
        .ok();
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        let field_name = translate_field_name(field.name());

        let debug_formatted = format!("{value:?}");
        write!(
            self.writer,
            " {}={:?}",
            decorate_field_name(field_name),
            quote_and_escape(&debug_formatted)
        )
        .ok();

        let display_formatted = format!("{value}");
        write!(
            self.writer,
            " {}.display={}",
            decorate_field_name(field_name),
            quote_and_escape(&display_formatted)
        )
        .ok();
    }
}

/// The type of record we are dealing with: entering a span, exiting a span, an event.
#[derive(Debug)]
pub enum Type {
    /// Starting span.
    EnterSpan,
    /// Exiting span.
    ExitSpan,
    /// Event.
    Event,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Type::EnterSpan => "START",
            Type::ExitSpan => "END",
            Type::Event => "EVENT",
        };
        write!(f, "{repr}")
    }
}

/// Translate the field name from tracing into the logfmt style.
fn translate_field_name(name: &str) -> &str {
    let name = slice_field_name(name);

    if name == "message" {
        "msg"
    } else {
        name
    }
}

/// Decorates field names in logs if the `ansi-logs` is on.
#[cfg(feature = "ansi-logs")]
#[inline]
fn decorate_field_name(name: &str) -> String {
    ansi_term::Color::Fixed(GRAY)
        .italic()
        .paint(name)
        .to_string()
}

/// Decorates field names in logs when `ansi-logs` is not on.
#[cfg(not(feature = "ansi-logs"))]
#[inline]
fn decorate_field_name(name: &str) -> String {
    name.to_string()
}

/// Converts system time to `rfc3339` format.
fn to_rfc3339(st: &SystemTime) -> String {
    st.duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .and_then(|duration| TryFrom::try_from(duration).ok())
        .and_then(|duration| OffsetDateTime::UNIX_EPOCH.checked_add(duration))
        .and_then(|dt| dt.format(&Rfc3339).ok())
        .unwrap_or_default()
}

/// Return true if the string value already starts/ends with quotes and is
/// already properly escaped (all spaces escaped).
fn needs_quotes_and_escaping(value: &str) -> bool {
    // mismatches beginning  / end quotes
    if value.starts_with('"') != value.ends_with('"') {
        return true;
    }

    // ignore beginning/ending quotes, if any
    let pre_quoted = value.len() >= 2 && value.starts_with('"') && value.ends_with('"');

    let value = if pre_quoted {
        &value[1..value.len() - 1]
    } else {
        value
    };

    // unescaped quotes
    let c0 = value.chars();
    let c1 = value.chars().skip(1);
    if c0.zip(c1).any(|(c0, c1)| c0 != '\\' && c1 == '"') {
        return true;
    }

    // Quote any strings that contain a literal '=' which the logfmt parser
    // interprets as a key/value separator.
    if value.chars().any(|c| c == '=') && !pre_quoted {
        return true;
    }

    if value.bytes().any(|b| b <= b' ') && !pre_quoted {
        return true;
    }

    false
}

/// Escape any characters in name as needed, otherwise return string as is.
fn quote_and_escape(value: &'_ str) -> Cow<'_, str> {
    if needs_quotes_and_escaping(value) {
        Cow::Owned(format!("{value:?}"))
    } else {
        Cow::Borrowed(value)
    }
}

/// slice / cut fields with a `.`.
fn slice_field_name(name: &str) -> &str {
    match name {
        name if name.starts_with("log.") => &name[4..],
        name => name,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quote_and_escape_len0() {
        assert_eq!(quote_and_escape(""), "");
    }

    #[test]
    fn quote_and_escape_len1() {
        assert_eq!(quote_and_escape("f"), "f");
    }

    #[test]
    fn quote_and_escape_len2() {
        assert_eq!(quote_and_escape("fo"), "fo");
    }

    #[test]
    fn quote_and_escape_len3() {
        assert_eq!(quote_and_escape("foo"), "foo");
    }

    #[test]
    fn quote_and_escape_len3_1quote_start() {
        assert_eq!(quote_and_escape("\"foo"), "\"\\\"foo\"");
    }

    #[test]
    fn quote_and_escape_len3_1quote_end() {
        assert_eq!(quote_and_escape("foo\""), "\"foo\\\"\"");
    }

    #[test]
    fn quote_and_escape_len3_2quote() {
        assert_eq!(quote_and_escape("\"foo\""), "\"foo\"");
    }

    #[test]
    fn quote_and_escape_space() {
        assert_eq!(quote_and_escape("foo bar"), "\"foo bar\"");
    }

    #[test]
    fn quote_and_escape_space_prequoted() {
        assert_eq!(quote_and_escape("\"foo bar\""), "\"foo bar\"");
    }

    #[test]
    fn quote_and_escape_space_prequoted_but_not_escaped() {
        assert_eq!(quote_and_escape("\"foo \"bar\""), "\"\\\"foo \\\"bar\\\"\"");
    }

    #[test]
    fn quote_and_escape_quoted_quotes() {
        assert_eq!(quote_and_escape("foo:\"bar\""), "\"foo:\\\"bar\\\"\"");
    }

    #[test]
    fn quote_and_escape_nested_1() {
        assert_eq!(quote_and_escape(r#"a "b" c"#), r#""a \"b\" c""#);
    }

    #[test]
    fn quote_and_escape_nested_2() {
        assert_eq!(
            quote_and_escape(r#"a "0 \"1\" 2" c"#),
            r#""a \"0 \\\"1\\\" 2\" c""#
        );
    }

    #[test]
    fn quote_not_printable() {
        assert_eq!(quote_and_escape("foo\nbar"), r#""foo\nbar""#);
        assert_eq!(quote_and_escape("foo\r\nbar"), r#""foo\r\nbar""#);
        assert_eq!(quote_and_escape("foo\0bar"), r#""foo\0bar""#);
    }

    #[test]
    fn not_quote_unicode_unnecessarily() {
        assert_eq!(quote_and_escape("mikuličić"), "mikuličić");
    }

    #[test]
    fn test_uri_quoted() {
        assert_eq!(quote_and_escape("/api/v2/write?bucket=06fddb4f912a0d7f&org=9df0256628d1f506&orgID=9df0256628d1f506&precision=ns"),
                   r#""/api/v2/write?bucket=06fddb4f912a0d7f&org=9df0256628d1f506&orgID=9df0256628d1f506&precision=ns""#);
    }
}
