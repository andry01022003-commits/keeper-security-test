//! The `LOG_FORMAT=json` diagnostics line (#2413 item 1, #4674).
//!
//! One JSON object per line. Unlike the stock `tracing-subscriber` JSON
//! format, which nests event fields under `fields` and span fields under
//! `span`/`spans`, every field a SIEM rule is likely to target is a top-level
//! key:
//!
//! * the event's own fields (`message`, plus e.g. `status`, `user`, `ip`,
//!   `latency_ms` on the per-request `Request completed` line), flattened as
//!   with `flatten_event(true)`;
//! * the fields of every span the event sits in, so the `http_request` root
//!   span's `method`, `uri`, `correlation_id`, `http.route`, ... appear on every
//!   line logged while the request is handled. On a name clash the OUTERMOST
//!   span wins, so an inner span's `uri` (an upstream URL, say) never masks the
//!   request's; and the event's own field wins over any span's.
//!
//! The full span stack is still emitted under `spans` (root first, each with its
//! `name`), so nothing the hoisting shadows is lost. `timestamp`, `level`,
//! `target` and `spans` are reserved: a field with one of those names never
//! overrides the envelope.
//!
//! The field reference lives in `docs/operations/logging.md`.

use std::fmt;

use serde_json::{Map, Value};
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::{JsonFields, Writer};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormattedFields};
use tracing_subscriber::registry::LookupSpan;

/// Envelope keys no event or span field may override.
const RESERVED: [&str; 4] = ["timestamp", "level", "target", "spans"];

/// The flat JSON event format. Paired with [`JsonFields`] (what
/// `fmt::layer().json()` installs), which stores each span's recorded fields
/// as a JSON object string.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FlatJson;

impl<S> FormatEvent<S, JsonFields> for FlatJson
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, JsonFields>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let line = flat_json_line(ctx, event);
        writeln!(writer, "{line}")
    }
}

fn flat_json_line<S>(ctx: &FmtContext<'_, S, JsonFields>, event: &Event<'_>) -> String
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let mut fields = EventFields::default();
    event.record(&mut fields);

    let mut top = Map::new();
    let mut spans = Vec::new();
    if let Some(scope) = ctx.event_scope() {
        for span in scope.from_root() {
            let span_fields = span
                .extensions()
                .get::<FormattedFields<JsonFields>>()
                .and_then(|f| serde_json::from_str::<Map<String, Value>>(f).ok())
                .unwrap_or_default();
            for (key, value) in &span_fields {
                // Outermost span wins on a clash.
                top.entry(key.clone()).or_insert_with(|| value.clone());
            }
            let mut entry = Map::with_capacity(span_fields.len() + 1);
            entry.insert("name".into(), Value::from(span.name()));
            entry.extend(span_fields);
            spans.push(Value::Object(entry));
        }
    }
    // The event's own fields win over any span's.
    top.extend(fields.values);
    for key in RESERVED {
        top.remove(key);
    }

    let meta = event.metadata();
    let mut line = Map::with_capacity(top.len() + RESERVED.len());
    line.insert(
        "timestamp".into(),
        Value::from(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true)),
    );
    line.insert("level".into(), Value::from(meta.level().as_str()));
    // A `log`-crate record arrives with target "log" and its real target in
    // `log.target`; report the real one.
    line.insert(
        "target".into(),
        Value::from(fields.log_target.as_deref().unwrap_or(meta.target())),
    );
    line.extend(top);
    if !spans.is_empty() {
        line.insert("spans".into(), Value::Array(spans));
    }
    serde_json::to_string(&line).unwrap_or_else(|_| {
        // Every value above is a string, number, bool or nested map of those,
        // so serialisation cannot fail; keep a line rather than drop it.
        r#"{"level":"ERROR","message":"log line failed to serialise"}"#.to_string()
    })
}

/// Collects an event's fields as JSON values.
#[derive(Default)]
struct EventFields {
    values: Map<String, Value>,
    /// `log.target` of a `log`-crate record bridged by `tracing-log`.
    log_target: Option<String>,
}

impl EventFields {
    fn insert(&mut self, field: &Field, value: Value) {
        let name = field.name();
        if name == "log.target" {
            if let Value::String(s) = value {
                self.log_target = Some(s);
            }
            return;
        }
        // The rest of the `tracing-log` metadata (`log.module_path`,
        // `log.file`, `log.line`) is noise on a diagnostics line.
        if name.starts_with("log.") {
            return;
        }
        let name = name.strip_prefix("r#").unwrap_or(name);
        self.values.insert(name.to_string(), value);
    }
}

impl Visit for EventFields {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.insert(field, Value::from(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.insert(field, Value::from(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.insert(field, Value::from(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.insert(field, Value::from(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.insert(field, Value::from(value));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.insert(field, Value::from(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.insert(field, Value::from(format!("{value:?}")));
    }
}
