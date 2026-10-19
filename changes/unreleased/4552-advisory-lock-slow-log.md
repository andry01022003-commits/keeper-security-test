---
section: Fixed
issues: [#4552]
---
- **A replica waiting on another replica's proxy single-flight lock no longer logs a sqlx "slow statement" WARN** (#4552). With `PROXY_SINGLEFLIGHT_ADVISORY_LOCKS_ENABLED=true`, every follower's `SELECT pg_advisory_lock($1, $2)` blocks until the leader finishes its fill, which is the designed behaviour, but it ran on a connection with sqlx's default 1 s slow-statement threshold, so each waiting replica logged one WARN per cold object (13 s for a 256 MiB fill, 65 s at the wait timeout) and buried real slow queries. The wait now runs on a dedicated session opened from the pool's connect options with slow-statement logging off (its statements still log at DEBUG like any other), and the outcome, granted or expired, is logged once at DEBUG with the lock key and `waited_secs`.
