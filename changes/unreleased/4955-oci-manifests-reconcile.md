---
section: Changed
issues: [#4955, #1683]
---
- **The OCI manifest existence table is now reconciled at startup and every six hours** (#4955, #1683). `oci_manifests` rows were written only by the push and delete handlers, so a tag removed by the lifecycle tag prune, by the startup orphan-tag reconciliation or by a 1.11.x replica during a rolling upgrade left a row for a manifest that answers 404 by digest, and a push served by a pre-268 replica left none. A reconcile pass now adds the missing rows and removes rows for manifests with no tag, no live index edge and no live `artifacts` row, re-checking each under its row lock. It runs at startup (replacing the one-shot backfill, which it includes) and every `OCI_MANIFESTS_RECONCILE_INTERVAL_SECS` (default 21600, `0` for startup only), and reports `ak_oci_manifests_reconcile_*` metrics. Nothing reads the table yet; this is the prerequisite for moving manifest reads onto it. See `docs/operations/oci-manifests-reconcile.md`.
