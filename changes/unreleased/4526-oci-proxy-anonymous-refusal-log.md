---
section: Fixed
issues: [#4526]
---
- **Pulling an image that does not exist through an anonymous OCI Remote no longer logs a "Proxy fetch failed" WARN on every pull** (#4526). Docker Hub answers a missing repository with `401`, so an anonymous Remote's manifest request is refused; the client correctly gets `404 MANIFEST_UNKNOWN` and the refusal is logged once at INFO, but the generic catch-all of the proxy error mapping also logged an unsampled WARN for the same pull, so a client retrying a missing image filled the log with WARNs. An upstream `401`/`403` to a request that carried no credentials (no upstream credentials configured, or a refused anonymous bearer token) now logs that line at DEBUG. Refusals of a credentialed request, upstream `5xx` and transport failures still log at WARN.
