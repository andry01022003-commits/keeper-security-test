# Diagnostics logging

The backend writes its diagnostics log (request lines, warnings, errors) to
stdout through `tracing`. Two environment variables shape it, both read once at
startup:

| Variable | Values | Default | Effect |
|---|---|---|---|
| `LOG_FORMAT` | `text`, `json` (`pretty` is accepted as an alias for `text`) | `text` | Line format. |
| `LOG_ANSI` | `auto`, `true`, `false` | `auto` | ANSI colour in the `text` format. |

`RUST_LOG` still selects which events are logged (default
`artifact_keeper_backend=debug,tower_http=debug,sqlx::query=info`). The audit
stream (`AUDIT_STREAM`) is separate; see [audit-events.md](../audit-events.md).

## ANSI colour (`LOG_ANSI`)

Before 1.11.2 the `text` format always coloured its output, including in a
container whose stdout is a pipe. The escape codes sit between a key and its
value (`uri` `ESC[0m=` `/api/...`), so a plain string filter such as
`|= "status=500"` in Loki, or any line-oriented SIEM rule, never matched (#4674).

- `auto` (the default) colours only when stdout is a terminal and `NO_COLOR`
  is unset or empty. Under Kubernetes, Docker or systemd the output is
  uncoloured with no setting needed.
- `true` forces colour on, whatever stdout is (and overrides `NO_COLOR`); the
  formatter is the one earlier releases used, so the output matches theirs
  apart from the fields 1.11.2 adds to the `Request completed` line.
- `false` forces colour off.

Any other value is treated as `auto`. `LOG_ANSI` has no effect on the `json`
format, which never contains escape codes.

## JSON format (`LOG_FORMAT=json`)

One JSON object per line. Every field is a top-level key, so a SIEM or Loki
`| json` stage can address it directly:

```json
{"timestamp":"2026-10-17T14:12:29.509037Z","level":"INFO","target":"artifact_keeper_backend::api::middleware::tracing","otel.kind":"server","method":"GET","uri":"/api/v1/repositories/npm-local/download/a.tgz","correlation_id":"3d0f6a52-...","http.route":"/api/v1/repositories/:key/download/*path","message":"Request completed","status":200,"user":"alice","ip":"203.0.113.7","latency_ms":42,"spans":[{"name":"http_request","otel.kind":"server","method":"GET","uri":"/api/v1/repositories/npm-local/download/a.tgz","correlation_id":"3d0f6a52-...","http.route":"/api/v1/repositories/:key/download/*path"}]}
```

### Field reference

Envelope, on every line:

| Field | Type | Notes |
|---|---|---|
| `timestamp` | string | RFC 3339, UTC, microseconds. |
| `level` | string | `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`. |
| `target` | string | Rust module path that logged the event. |
| `message` | string | The human-readable message. Do not parse it; match on the fields below. |
| `spans` | array | The span stack the event was logged in, root first, each `{ "name": ..., <its fields> }`. Omitted outside any span. |

Request fields, on every line logged while an HTTP request is being handled
(they come from the `http_request` root span):

| Field | Type | Notes |
|---|---|---|
| `method` | string | HTTP method. |
| `uri` | string | Request path and query, with credential-bearing query parameters replaced by `[REDACTED]` (#544). |
| `correlation_id` | string | The `X-Correlation-ID` echoed to the caller; also the `correlation_id` of audit records (#2414), so it joins the two streams. |
| `http.route` | string | Matched route pattern, e.g. `/api/v1/repositories/:key`. Absent when no route matched. |
| `otel.kind` | string | Always `server`; an OpenTelemetry hint, safe to ignore. |

The per-request summary line, `"message":"Request completed"`, logged once
per request at `INFO` when the response is ready, adds:

| Field | Type | Notes |
|---|---|---|
| `status` | integer | HTTP response status code. |
| `user` | string | Username the request authenticated as. **Absent** for an anonymous request. |
| `ip` | string | Client IP, resolved from the TCP peer; `X-Forwarded-For` is believed only from peers inside `RATE_LIMIT_TRUSTED_PROXY_CIDRS`. Absent when no address could be resolved. |
| `latency_ms` | integer | Time from the request reaching the correlation middleware to its response, in milliseconds. |

Write rules against `"message":"Request completed"` lines: they are the only
lines guaranteed to carry `status`, `user`, `ip` and `latency_ms`. Successful
health-probe requests (`/health`, `/healthz`, `/ready`, `/readyz`, `/livez`)
are not logged unless `LOG_PROBE_REQUESTS` is set.

Other events add their own fields as top-level keys too (e.g. `error`). Where
a field name appears more than once, the event's own field wins over a span's,
and an outer span wins over an inner one, so `uri` is always the request's
URI and never, say, the upstream URL of a proxy fetch inside it; the inner
span's fields remain under `spans`. `timestamp`, `level`, `target` and
`spans` are reserved and always mean the envelope.

`user` is recorded by the shared authentication middleware (all `/api/v1`
routes and the native package-format routes) and by the OCI `/v2` credential
check. A format handler that verifies a credential on its own, outside those
paths, logs the request without `user`.

### Changes from 1.10 and 1.11.0/1.11.1

`LOG_FORMAT=json` was introduced in 1.10.0 (#2413) with the stock
`tracing-subscriber` layout. 1.11.2 flattens it (#4674):

| Before | Now |
|---|---|
| `fields.message`, `fields.<name>` | `message`, `<name>` at the top level |
| `span.<name>` (current span) | the root `http_request` span's fields at the top level; the full stack under `spans` |
| `spans[]` | unchanged shape (`name` plus fields per span) |
| `status` as a string (`"201"`) | `status` as an integer (`201`) |

Update any parser that read `fields.message` or `span.correlation_id`.

## Example queries

Loki, JSON format:

```logql
{app="artifact-keeper-backend"} | json | message = "Request completed" | status >= 500
{app="artifact-keeper-backend"} | json | message = "Request completed" | user = "alice" | line_format "{{.method}} {{.uri}} {{.status}} {{.ip}}"
```

Loki, text format (needs `LOG_ANSI` at its default or `false`):

```logql
{app="artifact-keeper-backend"} |= "Request completed" |= "status=500"
```

## Helm chart

The chart does not yet expose these two settings as values; set them with the
backend's extra environment variables until
[artifact-keeper-iac#387](https://github.com/artifact-keeper/artifact-keeper-iac/issues/387)
lands. With `LOG_ANSI` at its default, pods already log without colour.
