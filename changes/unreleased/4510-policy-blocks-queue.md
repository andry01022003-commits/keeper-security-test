---
section: Added
issues: [#4510]
---
- **`GET /api/v1/admin/holds/policy-blocks` lists what a scan policy refuses right now, and the quarantine queue shows where each hold came from** (#4510). The new queue is derived from the download gate's own decision, not from the stored quarantine stamp: proxied paths whose stored `vulnerable` verdict that repository's proxy scan gate refuses (verdicts are shared by digest, so a record-only repository caching the same bytes does not list them), then hosted artifacts that the gate's scan-policy evaluation refuses (`block_unscanned`, `block_on_fail`, severity, predicate policies), each with its violations. It pages by cursor because the hosted half is evaluated, not counted. `GET /api/v1/admin/holds/quarantine` items gain `hold_source` (`upload`, `admin`, `scan_policy` or `null`) and the queue takes a `hold_source` filter. Instance administrators with an unrestricted credential only. See docs/security/download-holds.md.
