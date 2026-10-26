---
section: Added
issues: [#4182]
---
- **Conda package requests that find the database pool saturated are shed at once with `503` and `Retry-After: 1`** (#4182). A package request used to queue on the pool for up to `DATABASE_ACQUIRE_TIMEOUT_SECS` (5 s) before it was answered 503, so a burst from a fleet of installers (rattler fetches up to 50 packages at once per installer) stalled every queued request before telling it to back off. The package route's lookup now waits at most `CONDA_PACKAGE_DB_WAIT_MS` (default 500, `0` turns the gate off) for a connection and otherwise answers 503 with `Retry-After: 1` straight away, which conda clients retry; shed requests are counted in `ak_conda_package_requests_shed_total`. `docs/operations/load-shedding.md` has pool, Postgres and concurrency sizing for an installer fleet.
