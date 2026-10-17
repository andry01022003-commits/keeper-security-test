---
section: Fixed
issues: [#4759]
---
- **A Remote whose upstream promises a `Content-Length` and sends no body now answers `502 UPSTREAM_SHORT_BODY` instead of dropping the client connection with no status** (#4759). The streaming proxy returned the upstream's `200` and `Content-Length` before reading any of the body, so when the body never came the only thing left to do was close the connection (curl: "Empty reply from server").

  The streaming path now waits for the first body byte before it answers. If the upstream body ends (or fails) before that byte, the client gets a `502` with code `UPSTREAM_SHORT_BODY` and a message naming the repository and the expected and received byte counts, or the stale cached copy when stale-if-error has one. Once bytes have been forwarded the response is committed, so a body that ends short after that point still closes the connection, which is the only honest signal left; a warn now names the cache key with the expected and received counts. In both cases nothing is cached, as before.
