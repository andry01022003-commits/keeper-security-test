---
section: Added
issues: [#4337]
---
- **Logs and metrics can be exported over OTLP next to traces** (#4337). Set `OTEL_LOGS_EXPORTER=otlp` to send the diagnostics log to the collector as OpenTelemetry log records, each carrying the `trace_id` and `span_id` of the span it was emitted under, and `OTEL_METRICS_EXPORTER=otlp` to push the `/metrics` instruments with the same names and labels. Both are off by default, use the protocol and collector of `OTEL_EXPORTER_OTLP_ENDPOINT` (or `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` / `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`), and leave stdout logging and the Prometheus `/metrics` endpoint unchanged. SQL text stays stdout-only (#4455). Every OTLP signal now carries `service.instance.id`; `OTEL_TRACES_EXPORTER=none` turns span export off. See `docs/operations/logging.md`.
