---
section: Fixed
issues: [#4829]
---
- **A remote conda channel serves a CEP-16 shard again once its upstream does, after refusing it for not matching its address** (#4829). Since 1.11.0 a shard whose upstream bytes do not hash to its address is answered with 502, but those bytes had already been written to the proxy cache under the shard's path, which is cached as immutable and never revalidated. After the upstream recovered, the remote, and every virtual over it, kept answering 502 for that shard without asking the upstream again until the cache entry was evicted by hand.

  A shard is now checked against its address before it is committed to the proxy cache, and only a shard that matches is stored. A cached shard that does not match (one stored by an earlier release) is evicted on the next request and fetched again, so an affected cache heals without operator action. Both cases count in `ak_proxy_cache_checksum_mismatch_total`.
