---
section: Fixed
issues: [#4639]
---
- **The permissions API now names group, project and artifact targets instead of returning `target_name: null`** (#4639). `GET /api/v1/permissions`, `GET /api/v1/permissions/{id}` and the create/update responses resolved `target_name` for repository targets only, so the web UI's permissions page showed raw UUIDs for every other grant. The same single query now also joins the target group's name, the target project's name, and an artifact target's `name@version` (its path when it has no version). A grant whose target was deleted, including a soft-deleted artifact, still reports `null`.
