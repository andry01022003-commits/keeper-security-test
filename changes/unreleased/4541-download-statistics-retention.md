---
section: Added
issues: [#4541]
---
- **Download statistics can be trimmed on a schedule with `DOWNLOAD_STATS_RETENTION_DAYS`** (#4541). Every download writes one row to `download_statistics` (hosted) or `proxy_download_statistics` (proxy), and since proxy history outlives cache eviction (#4540) neither table shrank except on repository delete. Set `DOWNLOAD_STATS_RETENTION_DAYS` to a positive number of days and a scheduler pass deletes older rows from both tables every `DOWNLOAD_STATS_RETENTION_INTERVAL_SECS` (default one hour), on one replica at a time under a scheduler lease, in batches of 5000 rows. Off unless set. Download counts, trends, the `no_downloads_days` lifecycle policy and blast-radius attribution forget what the period drops; `docs/operations/download-statistics-retention.md` explains how to choose it.
