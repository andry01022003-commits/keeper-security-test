---
section: Fixed
issues: [#4892]
---
- **Concurrent first Maven uploads of one immutable release coordinate can no longer leave the winner's row pointing at the loser's bytes** (#4892). The artifact row is inserted under the per-key publish lock before the object is written, so the upload that loses the immutable-coordinate `ON CONFLICT ... WHERE false` returns 409 without ever touching the winner's storage key; previously both uploads wrote the same object before the row insert, and a loser whose write landed last replaced the winner's bytes while its own row was rejected.
