---
section: Changed
issues: [#4173, #4172]
---
- **Hosted conda channels store their CEP-16 shards when packages change, and a shard request is a lookup by address** (#4173, #4172). Every shard request used to list the whole repository and rebuild shard after shard until one matched the requested hash, so resolving a 3,000-package environment cost O(names) catalog walks per shard. Shards now live in the new `conda_shard_index` table: database triggers mark the package names a write touches (upload, withdrawal, delete, quarantine change, metadata edit) in the writer's own transaction, only those names are rebuilt, and readers rebuild anything still marked before they serve. A stored shard is checked against its address before it is served. Channels that existed before the upgrade are backfilled in the background in resumable batches; until a channel is done its shards are built per request as before. See `docs/operations/conda-hosted-repodata.md`.
