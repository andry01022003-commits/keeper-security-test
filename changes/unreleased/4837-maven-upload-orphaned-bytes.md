---
section: Fixed
issues: [#4837]
---
- **Remove orphaned Maven artifacts left after failed uploads on object storage** (#4837). A hosted upload that writes bytes to storage before its database transaction commits no longer leaves them downloadable; failed publishes are cleaned up so rejected deploys return 404.
