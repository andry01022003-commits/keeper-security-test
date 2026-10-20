---
section: Fixed
issues: [#4805]
---
- **Streamed uploads above ~4.88 GiB no longer fail on S3-compatible stores that cap a multipart upload at 1,000 parts** (#4805). Part sizes assumed AWS's 10,000-part cap, so a large OCI layer pushed to such a store failed at part 1,001. The new `S3_MAX_PARTS` (default 10,000, clamped into 1 to 10,000) sets the part budget that multipart part sizes and part-count checks derive from; `S3_MAX_PARTS=1000` streams objects up to ~500 GiB. The default behaviour is unchanged.
