---
section: Fixed
issues: [#4341]
---
- **npm tarball downloads served directly from a Remote repository without scan-on-proxy are counted again** (closes #4341). The unscanned direct-Remote tarball path streamed the package without recording a proxy download, so download statistics, and the popularity and curation signals built on them, undercounted npm proxy traffic; the same tarball served through a Virtual, or with scan-on-proxy enabled, was counted. That path now records one download per GET under the canonical tarball path (never on HEAD), off the response path like Maven's. A last-known-good tarball a Virtual serves from an age-gated member is counted under that member too. The proxy-download recording check (#3446) now sees npm's direct-Remote streaming call, and no longer accepts a serve path as recorded only because the same function also calls the scan-on-proxy wrapper, which records its own serve and nothing else.
