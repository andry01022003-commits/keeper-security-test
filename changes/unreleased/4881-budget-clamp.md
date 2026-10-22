---
section: Fixed
issues: [#4881]
---
- **`AK_PROXY_METADATA_BUDGET_BYTES` above 4 GiB is now honoured instead of being silently clamped, so the conda virtual merge cap grows with it** (#4881). The buffered-metadata budget counted one semaphore permit per byte, and a reservation is taken through tokio's `acquire_many`, which takes a `u32`, so the budget was clamped to 4294967295 bytes with no log line: an operator who set 32 GiB got a 4 GiB budget, and the default `CONDA_VIRTUAL_MAX_CONCURRENT_MERGES`, derived from it, stayed at 3. The budget is now accounted in 4 KiB units (a reservation rounds up to a whole unit), so any value the platform can address is represented and 32 GiB yields `budget_bytes: 34359738368` and 28 default merges at the default member ceilings. A blank, non-numeric or zero value still falls back to the 1 GiB default, and that, or a value too large to represent, now logs a warning at startup naming the configured and effective values.
