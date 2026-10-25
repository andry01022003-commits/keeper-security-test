---
section: Changed
issues: [#4323]
---
- **Hosted OCI blob pulls verify the stored bytes against their digest** (Part of #4323). A layer or config blob whose stored object no longer hashes to its digest is cut short instead of served to completion, as format-native downloads already are.
