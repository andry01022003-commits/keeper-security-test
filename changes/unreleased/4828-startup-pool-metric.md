---
section: Fixed
issues: [#4828]
---
- **`ak_db_pool_connection_failures_total{stage="startup"}` is now exported** (#4828). The startup connect retry added in 1.11.2 (#4549) counted every failed initial connect attempt, but the database pool was built before the Prometheus recorder was installed, so those increments went to the `metrics` crate's no-op default recorder and the `startup` series never appeared on `/metrics`. The recorder is now installed before the pool is built, so a replica that needed several attempts to reach the database reports them once it is up. The `request` series was not affected.
