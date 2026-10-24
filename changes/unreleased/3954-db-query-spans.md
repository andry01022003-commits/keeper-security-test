---
section: Added
issues: [#3954]
---
- **`DB_QUERY_SPANS=all` exports a database span for every SQL statement without logging every statement** (#3954). Database `CLIENT` spans used to come only from the statements `RUST_LOG` let through, so a span per statement needed `sqlx::query=debug`, which also printed every statement on stdout. With `DB_QUERY_SPANS=all` the filter is applied to each output separately: every statement becomes a span while stdout and exported log records still follow `RUST_LOG`. `log` (the default) keeps the 1.11 behaviour and `off` turns database spans off. The startup line reports the mode as `db_query_spans`. See `docs/operations/logging.md`.
