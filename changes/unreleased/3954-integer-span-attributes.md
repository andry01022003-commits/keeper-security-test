---
section: Fixed
issues: [#3954]
---
- **Upstream `CLIENT` spans export `http.response.status_code` and `server.port` as integers** (#3954). They were recorded as unsigned integers, which the OpenTelemetry tracing bridge exports as strings, so a trace backend could not compare them numerically (`http.response.status_code >= 500`) or group them with the integer values the request span already exports.
