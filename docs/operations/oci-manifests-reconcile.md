# OCI manifest existence table: reconcile

`oci_manifests` records each OCI manifest a repository holds, independent of
its tags (#1683, first slice #4433). Every push, proxy cache fill and
migration import writes its row in the same transaction as the tag, and a
delete by digest or a delete of the last tag removes it (#4449). Nothing reads
the table yet: manifest GET/HEAD/DELETE by digest, the referrers API and the
storage-GC orphan predicate still derive existence from `oci_tags`, index
edges and `artifacts` rows. Switching those readers to the table is the next
slice of #1683, and it waits until the reconcile below has run clean on
populated instances.

## Why a reconcile

Some changes reach the reference tables without passing through the writers
that keep `oci_manifests` exact:

- a 1.11.x replica serving pushes or deletes during a rolling upgrade (a
  pre-268 replica writes no row at all);
- the lifecycle tag prune, which removes the tags of expired images;
- the startup orphan-tag reconciliation of migrated repositories.

Without a repair, the table would claim manifests that answer 404 by digest,
or miss manifests that serve fine.

## What it does

Each pass, under a cluster-wide advisory lock (one replica at a time):

1. **Adds** a row for every manifest that is tagged, or is a locally stored
   child of an index, and has no row. The body is read from storage and its
   digest verified first, exactly as the startup backfill does.
2. **Removes** a row when the manifest is no longer addressable: no tag points
   at it, no index edge names it as a child while that index is tagged or has
   a live `artifacts` row, and no live `artifacts` row in its repository
   stores it (by `oci-manifests/<digest>` key or, for an older migrated
   manifest, by checksum). Rows touched in the last 10 minutes are left alone.
   Each removal re-checks the condition under the row's lock, so a push that
   re-tags the digest in between keeps its row.

Removing a row deletes no storage object: storage GC decides about the bytes
from the reference tables, as before.

## Schedule

The pass runs at startup (it includes the backfill that used to run alone)
and then every `OCI_MANIFESTS_RECONCILE_INTERVAL_SECS` seconds, default 21600
(6 hours). `0` keeps only the startup pass. The value is read once at
startup, so changing it takes a restart. Every replica runs its own timer; a
pass that finds another replica holding the lock is skipped, not queued.

The removal scan walks `oci_manifests` in primary-key order and answers each
row with index lookups on `oci_tags`, `oci_manifest_refs` and `artifacts`, so
its cost grows with the table, not with `artifacts`: about two seconds for
300,000 rows when none is stale.

## Metrics

| Metric | Type | Meaning |
|---|---|---|
| `ak_oci_manifests_reconcile_rows_added_total` | counter | Rows added for manifests that had none. |
| `ak_oci_manifests_reconcile_rows_removed_total` | counter | Rows removed for manifests that are no longer addressable. |
| `ak_oci_manifests_reconcile_failures_total` | counter | Candidates that could not be added (unreadable or mismatching body) or rows that could not be removed. Retried next pass. |

A healthy instance settles at zero additions and removals per pass once the
upgrade is complete. Steady removals point at a tag-removal path that bypasses
the manifest writers; steady failures point at manifest bodies missing from
storage.
