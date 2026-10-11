---
section: Fixed
issues: [#4612]
---
- **A remote conda channel whose upstream publishes a CEP-16 shard index larger than 8 MiB now serves it instead of answering 502** (closes #4612). The shard index fetched from an upstream was capped at the 8 MiB general metadata tier, and conda-pypi's noarch index is 28.6 MB compressed, so a remote pointed at conda-pypi answered 502 for its shard index (and for every shard, which reads the index to find the upstream's shard location). An index past 8 MiB is now read again under its own 64 MiB ceiling, reserved from the buffered-metadata budget only for that second read, so channels with ordinary indexes (conda-forge's largest is about 1 MiB) keep the small reservation; the decoded ceiling is 128 MiB. An index past 64 MiB is treated as the upstream publishing no usable shard index: the reason is logged at `warn`, the remote answers 404 and so does a virtual over it, and clients fall back to `repodata.json`.
