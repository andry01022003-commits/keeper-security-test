---
section: Added
issues: [#4176, #4172]
---
- **Hosted conda repodata renders are bounded per process, stream the catalog, and shed with `503` past a wait** (#4176, #4172). A render of a hosted subdir's documents now walks the catalog with a cursor and writes each record straight into the served encodings, so its memory is about the size of its output instead of the whole row set. At most `AK_CONDA_REGEN_CONCURRENCY` renders (default 2) run at once per process, and a request that has waited `AK_CONDA_REGEN_WAIT_SECS` (default 30, `0` for no bound) for its render is answered `503` with `Retry-After: 5` while the render finishes for the requests that follow. New metrics `ak_conda_repodata_regenerations_in_flight` and `ak_conda_repodata_requests_shed_total`. See `docs/operations/load-shedding.md`.
