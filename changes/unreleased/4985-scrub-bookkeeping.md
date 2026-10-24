---
section: Fixed
issues: [#4985, #4323]
---
- **The storage scrub's run accounting and bookkeeping are accurate** (#4985, #4323). `objects_checked` no longer counts an object twice when recording its finding fails; bytes read from a repair donor and the re-read of a repaired object now count toward `bytes_read`, so `max_bytes` bounds a repair run too; each run removes findings whose artifact or OCI blob row was hard-deleted and the resumable cursors of deleted repositories; and a scrub requested for an unknown repository key is audited (`started`, then `failed`) before the 404.
