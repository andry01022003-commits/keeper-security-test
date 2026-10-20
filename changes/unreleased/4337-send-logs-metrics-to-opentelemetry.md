---
section: Added
issues: [#4337]
---
- **Logs and metrics are exported to OpenTelemetry when `OTEL_EXPORTER_OTLP_ENDPOINT` is set** (#4337). Traces already exported via `tracing-opentelemetry`; log records now go through a `tracing` layer attached to the OTLP logs pipeline with trace/span correlation, and business/HTTP metrics are fanned out to both Prometheus (`/metrics`) and OTLP. Each signal can be toggled independently with `OTEL_TRACES_EXPORTER`, `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` (`none` disables; default enables all when OTLP is configured). SQL text remains stdout-only (#4455).
