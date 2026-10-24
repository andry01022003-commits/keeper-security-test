---
section: Fixed
issues: [#4980]
---
- **An upload of content that storage GC is deleting at the same moment no longer commits a row whose object is gone** (#4980). GC's orphan-key sweep re-checks a key's references and then deletes the object; an upload that deduplicated against that object (or wrote it) and committed its row between the two left a live artifact whose downloads failed. The transaction that commits an upload's row now first takes the key's GC journal slot: while a GC pass holds a delete lease on the key (taken in the last 15 minutes, recorded in the new `oci_gc_candidates.leased_at` column, migration 302) it waits up to 5 seconds, writes the object again if a delete removed it, and otherwise answers a retryable `503` and commits nothing. Covers the generic PUT and multipart routes, the native uploads that go through the shared upload service, and chunked completion. See `docs/operations/storage-gc.md`.
