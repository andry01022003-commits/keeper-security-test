//! OTLP log export for `tracing` events (#4337).
//!
//! [`super::init_with_otel`] installs this layer next to the stdout `fmt`
//! layer, so a deployment that sets `OTEL_EXPORTER_OTLP_ENDPOINT` ships its
//! diagnostics log to the collector as OpenTelemetry log records, over the
//! same OTLP endpoint (and protocol) as spans and metrics. Stdout output is
//! unchanged.
//!
//! Every exported record carries the trace context of the span the event was
//! emitted under (`trace_id` / `span_id`), which is what makes a log line
//! correlateable with its span in a backend such as Grafana Tempo — the
//! problem behind #4337. The context is read from the OTel span
//! `tracing-opentelemetry` stored in the *tracing* span
//! ([`tracing_opentelemetry::get_otel_context`], the same mechanism
//! [`db_spans`] uses), not from the ambient `opentelemetry::Context`, which
//! stays empty in a `tracing`-driven application.
//!
//! What reaches the export:
//!
//! * exactly the events the global `EnvFilter` lets through for stdout — the
//!   layer sits behind that filter, like every other layer;
//! * everything except `sqlx::query` events
//!   ([`db_spans::exported_to_otel`]): those carry SQL text, which stays
//!   stdout-only (#4455). Slow statements still appear as `CLIENT` spans via
//!   [`db_spans::DbQuerySpanLayer`].
//!
//! The layer is installed only on the OTLP path, and only when
//! `OTEL_LOGS_EXPORTER` is not `none` (see [`super::SignalExporter`]).

use std::sync::OnceLock;

use opentelemetry::logs::{AnyValue, LogRecord as _, Logger as _, Severity};
use opentelemetry::trace::TraceContextExt;
use opentelemetry_sdk::logs::{SdkLogRecord, SdkLogger};
use tracing::dispatcher::WeakDispatch;
use tracing::field::{Field, Visit};
use tracing::{Dispatch, Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// Map a `tracing` level to the OTel severity that bears its name.
fn severity_of(level: &tracing::Level) -> Severity {
    match *level {
        tracing::Level::ERROR => Severity::Error,
        tracing::Level::WARN => Severity::Warn,
        tracing::Level::INFO => Severity::Info,
        tracing::Level::DEBUG => Severity::Debug,
        tracing::Level::TRACE => Severity::Trace,
    }
}

/// A `tracing` layer that forwards events to the OTLP logs pipeline.
pub(crate) struct OtelLogLayer {
    logger: SdkLogger,
    /// The dispatch this layer is part of, needed to look up the OTel context
    /// of a tracing span (`tracing_opentelemetry::get_otel_context`).
    dispatch: OnceLock<WeakDispatch>,
}

impl OtelLogLayer {
    pub(crate) fn new(logger: SdkLogger) -> Self {
        Self {
            logger,
            dispatch: OnceLock::new(),
        }
    }

    /// Attach the trace context of the span the event was emitted under, when
    /// there is one and it carries a valid OTel span. Events outside any span
    /// (background work, startup) are exported without one.
    fn record_trace_context<S>(
        &self,
        event: &Event<'_>,
        ctx: &Context<'_, S>,
        record: &mut SdkLogRecord,
    ) where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let Some(span) = ctx.event_span(event) else {
            return;
        };
        let Some(dispatch) = self.dispatch.get().and_then(WeakDispatch::upgrade) else {
            return;
        };
        let Some(cx) = tracing_opentelemetry::get_otel_context(&span.id(), &dispatch) else {
            return;
        };
        let span = cx.span();
        let span_context = span.span_context();
        if !span_context.is_valid() {
            return;
        }
        record.set_trace_context(
            span_context.trace_id(),
            span_context.span_id(),
            Some(span_context.trace_flags()),
        );
    }
}

impl<S> Layer<S> for OtelLogLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_register_dispatch(&self, subscriber: &Dispatch) {
        let _ = self.dispatch.set(subscriber.downgrade());
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // SQL text never leaves stdout (#4455); the same filter that keeps it
        // off spans keeps it out of log records.
        if !super::db_spans::exported_to_otel(event.metadata()) {
            return;
        }
        let meta = event.metadata();
        let severity = severity_of(meta.level());
        let mut record = self.logger.create_log_record();
        record.set_target(meta.target());
        record.set_event_name(meta.name());
        record.set_severity_number(severity);
        record.set_severity_text(meta.level().as_str());
        self.record_trace_context(event, &ctx, &mut record);
        event.record(&mut EventVisitor {
            record: &mut record,
        });
        self.logger.emit(record);
    }
}

/// Copy the event's fields onto the log record: `message` becomes the body,
/// every other field becomes an attribute with its `tracing` type preserved
/// where the OTel value model has a counterpart (the same mapping
/// `opentelemetry-appender-tracing` uses; complex types fall back to their
/// `Debug` form).
struct EventVisitor<'a> {
    record: &'a mut SdkLogRecord,
}

impl EventVisitor<'_> {
    fn set_body_from_debug(&mut self, value: &dyn std::fmt::Debug) {
        self.record.set_body(format!("{value:?}").into());
    }
}

impl Visit for EventVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.set_body_from_debug(value);
        } else {
            self.record
                .add_attribute(field.name(), AnyValue::from(format!("{value:?}")));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.record.set_body(AnyValue::from(value.to_owned()));
        } else {
            self.record
                .add_attribute(field.name(), AnyValue::from(value.to_owned()));
        }
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.record.add_attribute(field.name(), value);
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.record.add_attribute(field.name(), value);
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.record.add_attribute(field.name(), value);
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        // OTel has no unsigned 64-bit integer attribute; a value beyond
        // i64 is stringified rather than wrapped.
        match i64::try_from(value) {
            Ok(signed) => self.record.add_attribute(field.name(), signed),
            Err(_) => self
                .record
                .add_attribute(field.name(), AnyValue::from(value.to_string())),
        }
    }

    fn record_error(&mut self, _field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.record
            .add_attribute("exception.message", AnyValue::from(value.to_string()));
    }
}
