---
section: Fixed
issues: [#4643]
---
- **Hosted npm packages and Go modules now replicate to push and mirror peers** (#4643). `npm publish` and Go module uploads never queued a peer sync task, so subscribed peers never received them (older versions can be sent with the run-now sync, `POST /api/v1/peers/{id}/repositories/{repo_id}/sync`). An npm version whose metadata contains a NUL character is now rejected with 400 instead of being stored without its metadata.
