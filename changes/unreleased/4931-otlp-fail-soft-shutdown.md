---
section: Fixed
issues: [#4931]
---
- **OTLP export starts, and stops, cleanly when the collector setting is wrong or the collector is down** (#4931). An unparseable `OTEL_EXPORTER_OTLP_ENDPOINT` no longer aborts startup: span export is turned off with a message on stderr, as log and metric export already were. An empty `OTEL_EXPORTER_OTLP_ENDPOINT` now counts as unset instead of turning the export on with nowhere to send. On shutdown the span, log and metric pipelines flush at the same time, so an unreachable collector holds shutdown for about 5 s instead of about 15 s, inside the 10 s a container runtime grants before it kills the process.
