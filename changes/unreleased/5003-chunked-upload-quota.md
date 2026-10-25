---
section: Fixed
issues: [#5003]
---
- **Chunked uploads apply the storage quota unless the session is a trusted replication session** (#5003). The quota check when a chunked upload session is created and the quota admission when it completes now exempt a session only when it is a replication session of an administrator or service account, as other replication-specific behaviour on the chunked upload path already required.
