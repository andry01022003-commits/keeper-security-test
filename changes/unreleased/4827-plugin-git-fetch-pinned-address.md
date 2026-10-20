---
section: Security
issues: [#4827]
---
- **Plugin installs and reloads fetch git sources through the SSRF-guarded HTTP client, so the connection goes only to an address the outbound policy admits at connect time** (#4827, GHSA-p28r-8m4q-w9pg). Previously libgit2 resolved the source host again after the URL check. Plugin git fetches now also honor the `HTTP_PROXY`/`HTTPS_PROXY` environment and `CUSTOM_CA_CERT_PATH` like the other outbound requests.
