---
section: Fixed
issues: [#4330]
---
- **The OpenAPI spec no longer marks `curation_default_action` as required on `UpdateRepositoryRequest`** (#4330). The field is optional on `PATCH /api/v1/repositories/{key}` and omitting it leaves the stored value unchanged, but a `value_type = String` schema override published it as the only required property, so clients generated from the spec had to `GET` the repository and echo the value back on every update. The schema now declares it as an optional string.
