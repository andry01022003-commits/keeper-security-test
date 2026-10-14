---
section: Fixed
issues: [#4667]
---
- **`POST /api/v1/security/findings/external` refuses a NUL byte with 400 and never leaves an orphaned `completed` scan behind** (#4667). A `\0` in any string field of a submission reached the `scan_findings` insert and answered 500 `DATABASE_ERROR`, and because the `scan_results` row had already been committed `completed` in a separate statement, the failed import left a clean-looking verdict with no findings under it. Every string field of `IngestExternalFindingsRequest` and `ExternalFindingInput` now goes through the NUL-refusing deserializer (400 `VALIDATION_ERROR`, as #3713 did for the other body carriers), and the scan row and its findings are written in one transaction.
