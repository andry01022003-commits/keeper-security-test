---
section: Fixed
issues: [#5003]
---
- **Chunked uploads honour the repository's upload quarantine hold** (#5003). An artifact committed through `PUT /api/v1/uploads/{id}/complete` into a hosted repository with the upload quarantine enabled is now held, as an artifact uploaded through the format's native route is.
