---
section: Changed
issues: [#4542]
---
- **The security blast-radius report keeps proxy pulls attributed after the cache entry is purged** (#4542). Proxy downloads were attributed to a vulnerable digest only through the live cache entry they were served from, so evicting or purging the compromised object, the usual remediation, also erased who had pulled it. Each `proxy_download_statistics` row now records the `checksum_sha256` that was served (a cache hit records the cached digest, a cache fill fills the rows of the serves it fed when it commits), and the report counts pulls by that digest; affected digests and repositories now include ones that were pulled and later purged. Rows from before the upgrade are still attributed through their cache entry while it exists. Migrations 295 to 297 add the column and build its index online (`CREATE INDEX CONCURRENTLY`).
