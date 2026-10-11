---
section: Fixed
issues: [#4551]
---
- **A Remote download larger than the repository quota no longer leaves a 0-byte "cached" entry** (#4551). Such an object is streamed to the client without any proxy-cache write (#4543), but recording the download still inserted the `proxy_cache_artifacts` placeholder a cache miss gets, and with no cache write to fill it in, the catalog listed a 0-byte cached object that was never stored. The serve now marks the object as not cached for its own download recorder, which records the download against `(repository_id, path)` (#4539) without creating a catalog row, and links it to an existing row only when one is already there. Downloads of over-quota objects are still counted.
