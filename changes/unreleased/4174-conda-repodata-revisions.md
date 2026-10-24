---
section: Changed
issues: [#4174, #4172]
---
- **Hosted conda repodata is rendered once per change and served from the metadata revision cache** (#4174, #4172). `repodata.json`, `.zst`, `.bz2`, `current_repodata.json`, `run_exports.json`, the JLAP bootstrap, the shard index and `channeldata.json` were rebuilt from a full catalog scan on every request. They are now rendered together once per subdir revision (which moves with every upload, withdrawal, delete, quarantine change and lapsed hold) and kept, so concurrent cold requests share one render and warm requests read no catalog rows. Virtual channels key their merges on their hosted members' revisions instead of reading every hosted record per request. Renders report through `ak_repo_metadata_regenerations_total{format="conda"}`, and other replicas are told on a notification channel of their own (`ak_conda_repodata_invalidation_v1`), which 1.11.x replicas do not listen on. The served bytes are unchanged.
