---
section: Fixed
issues: [#4658]
---
- **A file uploaded again to the same path now reaches push and mirror peers instead of the peers keeping the first upload** (#4658). A re-upload to a versioning-enabled Generic or ML model repository, and an upload after a delete, keep the artifact id, so the upload only raised the priority of the peer's push task, which had already completed, and the new content was never sent. Such a push is now queued again with a fresh retry budget, a push that finishes after the artifact changed under it is queued again, a push waits for an undelivered delete of the same artifact, and a second delete of a re-uploaded artifact is sent again. Other uploads that keep the id, such as a Maven snapshot redeploy, are not re-sent, because the peer would answer 409.
