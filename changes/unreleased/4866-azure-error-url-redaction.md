---
section: Fixed
issues: [#4866]
---
- **Storage error messages carry the operation and the transport error, not the request URL** (#4866). When a request to Azure Blob Storage failed in transport (connection reset, timeout), the error returned to the API client and written to the log repeated the full request URL, including its query string. They now name only the operation and the transport error. The network errors of a Google Cloud Storage resumable upload, which repeated the upload session URL, are trimmed the same way.
