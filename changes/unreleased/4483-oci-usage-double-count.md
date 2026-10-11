---
section: Fixed
issues: [#4483, #4545]
---
- **Docker/OCI repository usage and storage quotas count each layer once** (#4483, #4545). An OCI manifest's `artifacts` row records the image size (config plus layers, #3601), and those same bytes are counted through `oci_blobs`, so `storage_used_bytes` (the repository header and list), `GET /repositories/{key}` and the usage ledger behind quotas counted every image at least twice, and once more for every extra tag: one reported repository held 1.10 GB of blobs and showed 4.69 GB used, and a capped Docker repository refused pushes well before its real bytes reached the cap. Usage and the ledger now leave out rows stored under `oci-manifests/` (the rule the storage panel already used, #3286); `artifacts.size_bytes` keeps the image size, so the tag listing, the packages catalog and the UI are unchanged. A manifest row is admitted at 0 bytes on push, which is now exactly what it adds to the ledger. The manifest documents themselves (a few KB each) are no longer counted toward usage.

  Migration 288 replaces the ledger trigger function and recomputes `hosted_bytes` for every repository that holds OCI manifest rows, so existing deployments are corrected on upgrade. The background reconciler and `POST /api/v1/admin/storage/ledger/backfill` use the same rule.
