# Hosted conda repodata: stored shards and revisions

From 1.12.0 a hosted (local or staging) conda channel no longer rebuilds its
index documents from the catalog on every request (#4172).

## What is stored

- **CEP-16 shards** (#4173). Each `(repository, subdir, package name)` has
  one stored shard, the `.msgpack.zst` document a CEP-16 client (pixi,
  rattler, mamba 2, conda with the sharded plugin) fetches as
  `shards/<sha256>.msgpack.zst`. It lives in the `conda_shard_index` table
  with its SHA-256. A shard request is one lookup by address; the bytes are
  checked against the address before they are served and carry
  `Cache-Control: public, max-age=31536000, immutable`. The shard index
  (`repodata_shards.msgpack.zst`) is serialised from the stored map.
- **A revision per subdir** (#4174). Every rebuild of a subdir bumps its
  revision. `repodata.json`, `.zst`, `.bz2`, `current_repodata.json`,
  `run_exports.json` and the bootstrap `repodata.json.jlap` are rendered
  together from one catalog walk the first time a revision is read, and kept
  in memory under it. `channeldata.json` is kept under the sum of the
  repository's subdir revisions. A request reads the revision (one statement
  against the conda state tables, no catalog rows) and serves the kept bytes
  when it matches; concurrent requests for a new revision share one render.
  The served bytes are unchanged from 1.11.

The kept documents are per process. Each replica renders a revision once.

## How changes reach them

Database triggers on `artifacts` and `artifact_metadata` record every
`(repository, subdir, name)` a write touches in `conda_shard_dirty`, in the
writer's own transaction, whatever made the write: a conda upload or
withdrawal, a generic delete, the quarantine console, a scanner verdict, an
attestation added to a package. Only the dirty names are rebuilt: their rows
are read, only shards whose bytes changed are rewritten, and the subdir's
revision moves. The conda upload and withdrawal routes rebuild right after
their write; any other writer leaves the rebuild to the next read, which
always rebuilds what is dirty before it serves. A read therefore never sees a
document older than the catalog.

An upload hold that lapses changes what a channel lists with no write at all.
The earliest pending `quarantine_until` of each channel is kept, and the
first read after it rebuilds the names whose hold lapsed.

Rebuilds announce themselves to the other replicas on their own
notification channel (`ak_conda_repodata_invalidation_v1`, event
`conda_repodata_invalidated`, the same listener connection as
`ak_cache_invalidation_v1`) so they free the superseded documents early.
Correctness does not depend on the event: every replica checks the revision
on every request. The channel is separate so that 1.11.x replicas, which do
not listen on it, are not affected during a rolling upgrade: on the shared
channel they would answer every unknown event with a conservative flush of
their authorization caches.

## Backfill of existing channels

The migration marks every hosted conda repository that exists at upgrade
time as not backfilled. At startup each 1.12 backend backfills them in
batches of 500 package names, one transaction per batch, saving a cursor with
each, so a restart resumes where it stopped and two replicas never work on
the same repository at once. Until a repository finishes, its shard index and
shards are built per request as in 1.11; its repodata documents use the
revision cache from the first request, as they do not depend on the backfill.
The log line `conda shard backfill finished` marks each repository.

## Metrics

The kept documents report through the shared metadata-cache metrics with
`format="conda"` and `scope` `subdir`, `shard_index` or `channel`; see
[repo-metadata-cache-metrics.md](repo-metadata-cache-metrics.md). A burst of
cold requests after one upload moves
`ak_repo_metadata_regenerations_total{format="conda",scope="subdir"}` by one.
