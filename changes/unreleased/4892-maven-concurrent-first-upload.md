---
section: Fixed
issues: [#4892, #4893]
---
- **Concurrent first Maven uploads of one immutable release coordinate can no longer leave the winner's row pointing at the loser's bytes** (#4892). The object is now written inside the publishing transaction while holding the per-key publish lock, and an upload that finds the coordinate already published under that lock returns 409 without touching the winner's storage key; previously both uploads wrote the same object before either row was inserted, and a loser whose write landed last replaced the winner's bytes while its own row was rejected. The write happens before the repository-wide quota and usage-ledger row locks are taken, so other uploads into the same repository do not wait on it. In Artifactory migration path mode a hosted Maven artifact upload now writes its object once instead of twice (#4893).
