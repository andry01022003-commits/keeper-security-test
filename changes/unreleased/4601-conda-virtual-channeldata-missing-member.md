---
section: Fixed
issues: [#4601]
---
- **A virtual conda channel's `channeldata.json` no longer fails with 502 when one remote member's upstream publishes no channeldata** (closes #4601). The merge treated a member's 404 for `channeldata.json` as a member failure, so the whole virtual's channeldata answered 502 even though every other member answered and `repodata.json` for the same virtual merged fine; tools that index a channel from its channeldata (ProGet) could not index it at all. A 404 now means that member contributes no entries, logged at `info`, exactly as a missing subdir already did for `repodata.json`. A member that answers 5xx, times out, is over the size cap or does not parse still fails the request loudly with 502 naming it, or is reported in `X-AK-Partial-Members` when the virtual opted into partial merges.
