---
section: Security
issues: [#4261]
---
- **A repository-restricted token keeps its restriction when it is refreshed** (#4261). The refresh token was signed without the access token's repository allow-list, and `POST /api/v1/auth/refresh` re-mints the access token from the refresh token's claims, so a restricted session came back unrestricted after one rotation. Refresh tokens now carry the allow-list along with the action-scope ceiling they already carried, on the interactive refresh path and on the OCI registry offline-token path alike. Refresh tokens issued before this release still carry no allow-list and refresh as before until they expire.
