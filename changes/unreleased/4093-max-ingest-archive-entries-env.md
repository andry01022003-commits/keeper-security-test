---
section: Added
issues: [#4093]
---
- **The upload archive entry-count cap is configurable with `MAX_INGEST_ARCHIVE_ENTRIES`** (#4093). The 10,000-entry ceiling that guards metadata extraction against entry-count bombs was a compile-time constant, so a legitimate package with more files, such as a Composer package of a paid WordPress plugin that ships its `vendor/` tree, was refused with no way to admit it. It is now read like its sibling `MAX_INGEST_DECOMPRESSED_BYTES`: a positive integer raises or lowers the cap for every format that uses it (Composer, Swift, Helm, Debian, conda, protobuf, ansible, Python wheels and sdists), and a blank, zero or non-numeric value keeps the default of 10,000. Both variables are now documented in `.env.example`.
