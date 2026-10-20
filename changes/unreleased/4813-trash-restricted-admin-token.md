---
section: Fixed
issues: [#4813]
---
- **The admin trash listing and restore now honour a repository-restricted admin token** (#4813). `GET /api/v1/admin/trash` lists only the token's repositories, and `POST /api/v1/admin/trash/{id}/restore` answers 404 for an artifact outside them, the same as an unknown id, as `DELETE /api/v1/admin/trash/{id}` already did since #4701.
