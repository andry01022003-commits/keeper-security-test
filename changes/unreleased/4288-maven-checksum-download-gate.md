---
section: Fixed
issues: [#4288]
---
- **Maven checksums of a quarantined or scan-policy blocked artifact now answer 404** (#4288). A `.sha1`, `.md5`, `.sha256` or `.sha512` request for a hosted artifact answered with the digest even while the artifact itself was refused, from a stored sidecar or computed from the stored bytes, on the direct hosted route, for a `-SNAPSHOT` alias and through a virtual repository. The checksum now gets the artifact's own download gate (quarantine, then scan policy): while the artifact is refused every checksum of it answers 404, and once it is released the checksums are served again. Checksums of proxied (remote) artifacts come from the upstream and are unchanged.
