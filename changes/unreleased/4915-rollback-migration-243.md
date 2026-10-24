---
section: Fixed
issues: [#4915]
---
- **The rollback guide now warns that a 1.11.x database cannot serve a 1.10.x backend for OCI pushes** (#4915). Migration 243 (1.11.0, #3851) replaced the unique key on `oci_upload_cleanup_keys` that the 1.10.x upload path names in `ON CONFLICT (storage_key)`, so after a rollback to 1.10.x with `SKIP_MIGRATIONS=true` every `docker push` failed with "there is no unique or exclusion constraint matching the ON CONFLICT specification". `docs/operations/rollback.md` now says so up front, lists migration 243 in a new table of releases that cannot be rolled back across, and widens its boundary condition to dropped or replaced unique constraints.
