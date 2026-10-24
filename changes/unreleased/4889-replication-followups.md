---
section: Fixed
issues: [#4889]
---
- **Label moves and late format metadata now reach push peers** (#4889). The protobuf `_labels` index was a zero-byte row that the sync worker had nothing to transfer, and the NuGet, PyPI, Debian and nested-layout RPM routes queued their push before writing the format `artifact_metadata`. The label index is now stored as a real JSON object (with a refreshable, content-addressed row) and queued like any other upload, and those four routes queue after their metadata is committed; the metadata-diff re-arm now covers every format, so a worker that finished a transfer before the document landed redelivers it. Client-uploaded `maven-metadata.xml` remains intentionally row-less and unpushed: peers regenerate it from the versioned artifacts they receive rather than accepting one deploy's snapshot over their own generated view.
