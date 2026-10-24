---
section: Changed
issues: [#4882]
---
- **A hosted conda subdir's `repodata.json.bz2` is rendered on its first request and kept for the revision** (#4882). bzip2 is the slowest encoding by far (1.5 s for a 26 MB document, 26 s for conda-forge `linux-64`) and the one clients ask for least, so it is no longer rendered with every revision; the first `.bz2` request renders it from the kept `.zst` document under the regeneration bound, and later ones are served the kept bytes. The file is unchanged: single-stream bzip2 at level 9. A lower level was measured to save under a fifth of the time for a tenth more bytes, and parallel multistream output was not adopted: pixi 0.81 (rattler), which reads `.bz2` when a channel has no `.zst`, fails on a multistream file, while conda 26.9, mamba 2.9 and micromamba 2.0 never request `.bz2`.
