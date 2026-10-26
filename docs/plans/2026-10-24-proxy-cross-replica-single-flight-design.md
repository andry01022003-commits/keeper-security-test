# Proxy single-flight across replicas: decouple the fill and publish an in-flight row (design note)

**Date**: 2026-10-24
**Status**: Proposed, for maintainer review. Nothing here is decided; no code
ships with this note (implementation is 1.13.0 material). The open questions
at the end need an answer first.
**Issues**: #4533 (this design), #4013 and #4532 (what 1.11.0 shipped), #4331 (HA
latency report), #2928 (per-object cache ceiling), #4660 (per-Remote cache cap)

## What ships today

A cold proxied GET elects a leader in two layers:

1. **In process** (`BufferedCoordinator`, `proxy_hydration.rs`): concurrent
   requests for the same cache key on one replica share one upstream fetch; the
   leader's `StreamHandle` is fanned out to followers.
2. **Across replicas**, opt-in with `PROXY_SINGLEFLIGHT_ADVISORY_LOCKS_ENABLED`
   (`AdvisoryLockCoordinator`, #4532): the in-process leader takes a Postgres
   advisory lock keyed by `cluster_lock::lease_object_id(lease_key)`, a 32-bit
   FNV hash. A losing replica polls the lock (`PROXY_SINGLEFLIGHT_LOCK_POLL_INTERVAL_MS`,
   200 ms) for up to `PROXY_SINGLEFLIGHT_LOCK_WAIT_TIMEOUT_SECS` (65 s), at most
   `PROXY_SINGLEFLIGHT_MAX_LOCK_WAITERS` (32) waits at once, and then re-checks
   the cache. The leader releases after its cache publish.

The leader's body is teed (`CachePersister::tee_stream`): upstream chunks go to
the client and, through a bounded channel (`TEE_CHANNEL_DEPTH` = 64 chunks of at
most 64 KiB), to a storage writer that stages the object, copies it onto the
live key and writes the sidecar.

## What that leaves on the table

- **One client paces the fill.** The tee only reads upstream as fast as the
  leader's client reads it (the channel is bounded and the client stream is the
  driver). A slow leader client slows the fill, and every remote waiter waits
  for it. Past the wait timeout each replica fetches for itself, so the floor is
  N upstream fetches for N replicas, not 1.
- **A follower on another replica cannot serve from an in-progress object.** It
  only ever sees the finished cache entry.
- **The lock is a 32-bit hash.** Unrelated keys can share a lock: extra
  waiting, harmless today, but it rules the lock out as a precise signal.

## Proposed direction

1. **Decouple the fill from the client.** The leader reads upstream at upstream
   speed into a spool (the staging object it already writes), and its own client
   reads from the spool instead of driving the tee. A slow client then delays
   only itself.
2. **Publish an in-flight row** keyed by the full cache key, not a hash:

   ```sql
   CREATE TABLE proxy_cache_inflight (
       cache_key        TEXT PRIMARY KEY,
       leader           TEXT NOT NULL,        -- replica identity
       spool_key        TEXT NOT NULL,        -- staging object being written
       content_length   BIGINT,               -- upstream Content-Length, when known
       bytes_committed  BIGINT NOT NULL DEFAULT 0,
       state            TEXT NOT NULL,        -- filling | published | failed
       heartbeat_at     TIMESTAMPTZ NOT NULL DEFAULT now()
   );
   ```

   The row replaces the advisory lock as the election (an `INSERT ... ON
   CONFLICT DO NOTHING` wins; a stale `heartbeat_at` lets a successor take over).
3. **Followers by backend class.**
   - Filesystem (one shared `STORAGE_PATH`): followers tail the spool with
     ranged reads up to `bytes_committed`, so a second replica starts streaming
     before the fill ends.
   - S3, GCS, Azure: a multipart or block upload is not readable until it is
     committed, so followers wait on the row (`state = published`) and serve
     from the cache entry, as today but without the hash collisions and with a
     fill that runs at upstream speed.
4. **Do not build "follower proxies the leader's stream across processes".** It
   moves the fan-out hop onto the network, still needs the row to find the
   leader, and couples two replicas' lifetimes.

## What it interacts with

- **Digest-gated commit** (GHSA-qxv7-p3mq-88fv, #2274): a spool a follower has
  tailed may still fail the digest gate at the end. A follower that tails must
  therefore either buffer until `published` or accept that its client received
  bytes that were then refused for the cache (the client verifies the digest
  itself, as it does for an uncached stream today). The second matches current
  semantics for the leader's own client.
- **Per-object ceiling** (#2928) and **per-Remote cap** (#4660): the spool is
  bounded by the same ceiling; the cap's admission check runs at election, and
  eviction after the publish, as now.
- **Scan-on-proxy**: the scanned path buffers the body and does not tee, so it
  is unchanged.

## Open questions

1. **Spool lifecycle on leader death.** Who deletes an orphaned spool, and when
   may a successor reuse the bytes already committed rather than refetch?
   (Proposal: a successor refetches from byte 0; the janitor deletes spools of
   rows whose heartbeat is older than the lease TTL.)
2. **Heartbeat write rate.** Updating `bytes_committed` per chunk is too much
   write traffic; per N MiB or per second? (Proposal: every 1 s or 8 MiB,
   whichever comes first; tailing followers read the spool's size, not the row,
   for progress.)
3. **Does the in-process `BufferedCoordinator` stay** as a first layer (no DB
   round trip for the common single-replica case), with the row only for
   cross-replica election? (Proposal: yes.)
4. **Opt-in or default.** The advisory-lock coordinator is opt-in today. Should
   the row-based coordinator be the default for multi-replica Helm installs?
