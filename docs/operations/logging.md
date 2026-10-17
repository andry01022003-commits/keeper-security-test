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

One JSON object per line. The layout `LOG_FORMAT=json` has written since
1.10.0 (#2413) is unchanged: every key below under "Existing keys" is still
emitted with the same nesting and types, so existing parsers keep working.
From 1.11.2 (#4674) each line also carries the request fields as **top-level
keys**; target those in new detection rules.

```json
{"timestamp":"2026-10-17T14:12:29.509037Z","level":"INFO","fields":{"message":"Request completed","correlation_id":"3d0f6a52-...","status":"200","user":"alice","ip":"203.0.113.7","latency_ms":42},"target":"artifact_keeper_backend::api::middleware::tracing","span":{"correlation_id":"3d0f6a52-...","http.route":"/api/v1/repositories/:key/download/*path","method":"GET","otel.kind":"server","uri":"/api/v1/repositories/npm-local/download/a.tgz","name":"http_request"},"spans":[{"correlation_id":"3d0f6a52-...","http.route":"/api/v1/repositories/:key/download/*path","method":"GET","otel.kind":"server","uri":"/api/v1/repositories/npm-local/download/a.tgz","name":"http_request"}],"method":"GET","uri":"/api/v1/repositories/npm-local/download/a.tgz","status":200,"correlation_id":"3d0f6a52-...","http.route":"/api/v1/repositories/:key/download/*path","user":"alice","ip":"203.0.113.7","latency_ms":42}
```

### Top-level request fields (target these)

Each key is present only when something in scope recorded it. The value is
the event's own field when it has one, else the outermost span's, so `uri` is
always the request's URI and never, say, the upstream URL of a proxy fetch
inside it.

| Field | Type | Present on | Notes |
|---|---|---|---|
| `method` | string | every line logged during a request | HTTP method. |
| `uri` | string | every line logged during a request | Request path and query, with credential-bearing query parameters replaced by `[REDACTED]` (#544). |
| `correlation_id` | string | every line logged during a request | The `X-Correlation-ID` echoed to the caller; also the `correlation_id` of audit records (#2414), so it joins the two streams. |
| `http.route` | string | every line logged during a request, once routed | Matched route pattern, e.g. `/api/v1/repositories/:key`. Absent when no route matched. |
| `status` | integer | `Request completed` | HTTP response status code. (The nested `fields.status` keeps its historical string form.) |
| `user` | string | `Request completed` | Username the request authenticated as. **Absent** for an anonymous request. |
| `ip` | string | `Request completed` | Client IP, resolved from the TCP peer; `X-Forwarded-For` is believed only from peers inside `RATE_LIMIT_TRUSTED_PROXY_CIDRS`. Absent when no address could be resolved. |
| `latency_ms` | integer | `Request completed` | Time from the request reaching the correlation middleware to its response, in milliseconds. |
| `request_id` | any | any line that records it | Hoisted when an event or span records a `request_id` field. |

The per-request summary line has `fields.message` equal to
`"Request completed"` and is logged once per request at `INFO` when the
response is ready. Write rules against it: it is the only line guaranteed to
carry `status`, `user`, `ip` and `latency_ms`. Successful health-probe
requests (`/health`, `/healthz`, `/ready`, `/readyz`, `/livez`) are not logged
unless `LOG_PROBE_REQUESTS` is set.

`user` is recorded by the shared authentication middleware (all `/api/v1`
routes and the native package-format routes) and by the OCI `/v2` credential
check. A format handler that verifies a credential on its own, outside those
paths, logs the request without `user`.

### Existing keys (unchanged since 1.10.0)

| Field | Type | Notes |
|---|---|---|
| `timestamp` | string | RFC 3339, UTC, microseconds. |
| `level` | string | `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`. |
| `fields` | object | The event's own fields, including `message` (the human-readable text; do not parse it). On `Request completed` it also holds `correlation_id`, `status` (a string), and from 1.11.2 `user`, `ip`, `latency_ms`. |
| `target` | string | Rust module path that logged the event. |
| `span` | object | The current (innermost) span: its `name` and recorded fields. Omitted outside any span. |
| `spans` | array | The span stack, root first, each with its `name` and recorded fields. Omitted outside any span. |

The top-level request keys duplicate values already present under `fields`,
`span` or `spans`; that copy is what lets a rule address them without
descending into the nesting.

## Example queries

Loki, JSON format:

```logql
{app="artifact-keeper-backend"} | json | fields_message = "Request completed" | status >= 500
{app="artifact-keeper-backend"} | json | fields_message = "Request completed" | user = "alice" | line_format "{{.method}} {{.uri}} {{.status}} {{.ip}}"
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
