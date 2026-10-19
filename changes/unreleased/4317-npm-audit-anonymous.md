---
section: Fixed
issues: [#4317]
---
- **`npm audit` and the audit step of `npm install` work anonymously against a public npm repository** (#4317). The repository visibility middleware treated every anonymous `POST` as a write and answered 401, so the advisory lookups at `/-/npm/v1/security/advisories/bulk` and `/-/npm/v1/security/audits/quick` failed with "Unable to authenticate" even though the metadata and tarball `GET`s worked without credentials. Those two paths are now classified as reads: an anonymous caller can reach them on any repository it may already read anonymously, a private repository still answers 401, and every other npm write (publish, dist-tag, unpublish) still requires authentication. Authenticated read-only members also no longer get 403 on them.
