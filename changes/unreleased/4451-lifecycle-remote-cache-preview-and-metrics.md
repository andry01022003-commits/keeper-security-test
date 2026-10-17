---
section: Fixed
issues: [#4451]
---
- **Scheduled sweeps of Remote proxy caches now appear in the scheduler summary and the `lifecycle` cleanup metric** (#4451). The scheduled path counted only `artifacts_removed`, so a policy scoped to a Remote repository — whose evictions are reported on the separate `proxy_cache` counters — logged nothing and recorded zero on `ak_cleanup_items_removed_total{type="lifecycle"}`, while a manual execute reported the same run correctly. The summary line and the metric now include `proxy_cache.entries_removed` and the bytes freed by artifacts and cache evictions alike, so a cache-only policy is visible on dashboards.

  The [lifecycle assignment guide](docs/lifecycle-policy-assignments.md) now also documents the create-disabled → preview → enable order (#4451): a newly created policy with no `cron_schedule` and no `last_run_at` is due on the next scheduler tick, so assigning an already-enabled policy to a Remote repository could evict cache entries before the preview that was meant to show them was read.
