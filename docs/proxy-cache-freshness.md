# Proxy cache freshness and the yank asymmetry

Remote and Virtual repositories cache upstream responses on disk. Every cached
response is classified as **mutable** or **immutable**, and the two classes
expire on radically different schedules. That split explains the one symptom
operators find surprising: an upstream **yank** makes a resolve-then-fetch
client (npm, pip, Maven, …) fail at *resolution* even though the package's
artifact is still cached and downloadable.

## Two lifetimes

| Class | Examples | Default lifetime | On expiry |
| --- | --- | --- | --- |
| **Mutable** | npm packument, PyPI simple index, `maven-metadata.xml`, OCI tag, Go `@v/list`, Cargo sparse index, conda `repodata.json`, RPM `repomd.xml` | 300 s (5 min) | revalidated against upstream, then refilled |
| **Immutable** | npm tarball (`…/-/<pkg>-<ver>.tgz`), wheel/sdist, `.jar`, `.crate`, OCI digest blob, RPM package | 315360000 s (~10 years) | never expires, never revalidated |

Classification is by **path shape**, not by format alone — see
`backend/src/services/cache_classifier.rs`. The bias is deliberate: a mutable
path misclassified as immutable would serve stale pointers forever, whereas the
reverse only costs a cheap revalidation.

## Revalidation and stale-if-error

When a mutable entry passes its TTL the proxy **revalidates** it with a
conditional request:

- **304 Not Modified** → the TTL is extended and the cached body is served.
- **200 with new content** → the body is refilled.
- **5xx, timeout or transport error** → **stale-if-error** (RFC 5861): the stale
  body is served for up to 3600 s (1 h) past expiry. This is the *only* case
  where expired content is served.
- **404 or 410** → **not** stale-if-error. A definitive "not here" is treated as
  a real answer and **deletes the cached body** (see below).

## Negative caching

A definitive upstream **404 / 410** on a cacheable path writes a
**negative-cache marker**: any cached positive body for that path is deleted,
and for the next 45 s (`NEGATIVE_CACHE_TTL_SECS`) reads answer `404` straight
from the marker, without contacting upstream. (An upstream 410 is cached the
same way and is also served as `404` — a gone object is the same statement, one
step more definite.) The marker is a hot-loop guard: a miss for a
not-yet-published artifact must not hammer the origin. If upstream is still 404
when the marker expires, the next request re-checks, gets 404 again, and
refreshes the marker, so the 404 persists for as long as upstream keeps saying
404.

## The yank asymmetry

Combining the two lifetimes produces a symptom that looks like a cache bug but
is a direct consequence of the design:

1. `npm install <pkg>` caches two things: the **packument** (`/<pkg>`, mutable,
   5 min) and the **tarball** (`/<pkg>/-/<pkg>-<ver>.tgz`, immutable, ~10 years).
2. The upstream registry **yanks** the version (or the whole package).
3. Within 5 minutes the packument's TTL elapses; the proxy revalidates; upstream
   answers **404**; the cached packument body is **deleted** and a negative
   marker is written.
4. `npm install` now fails at **resolution** — `404 Not Found - GET /<pkg>` —
   because the packument is gone.
5. The **tarball still serves** (immutable), so fetching the exact tarball URL
   still returns `200` and downloads the bytes.

So the package manager reports "not found" while Artifact Keeper still holds the
artifact. The metadata and the artifact expired on different schedules — that is
the entire story. It is expected behaviour, not cache corruption, and it is
format-general: any format whose mutable metadata points at immutable artifacts
behaves this way (PyPI simple index vs wheel, `maven-metadata.xml` vs `.jar`,
OCI tag vs digest blob, Cargo index vs `.crate`, conda repodata vs package, …).

## What to do about it

- **Resolution vs download.** A failed resolution does not mean the artifact is
  unavailable. Given the exact tarball/artifact URL, fetch it directly —
  immutable entries keep serving.
- **`npm ci` with a lockfile.** `npm ci` installs straight from the lockfile, so
  it can fetch the already-pinned tarball URLs (which still serve) without
  re-resolving the packument. It is `npm install` — which may re-resolve
  metadata — that surfaces the yank.
- **After an upstream re-publish.** Resolution recovers automatically as soon as
  upstream answers again. While upstream keeps returning 404, the marker is
  refreshed every 45 s and resolution keeps failing — which is correct: upstream
  says the version is gone.
- **Force a re-check now.** `POST /api/v1/repositories/{key}/cache/invalidate?path=<path>`
  evicts a single entry, including a negative marker, so the next request
  re-resolves immediately. It needs the `write:repositories` scope, plus `admin`
  on the repository when the caller is not an administrator.
- **The TTL override applies to mutable paths only.** `PUT /api/v1/repositories/{key}/cache-ttl`
  with `cache_ttl_seconds` (1 … 315360000) changes the mutable lifetime;
  immutable artifacts are cached forever regardless. Lowering it surfaces a yank
  *sooner*; the maximum makes mutable paths effectively never revalidate, which
  avoids surfacing the yank only by knowingly serving a stale pointer. It cannot
  make immutable artifacts expire.

Serving the stale index while the corresponding artifacts are still held is a
deliberate behaviour change — it would contradict the "a definitive 404 deletes
the cached body" invariant — and is a separate design discussion, not a fix to
the cache.

## See also

- `backend/src/services/cache_classifier.rs` — classification rules and TTL
  constants (`MUTABLE_DEFAULT_TTL_SECS`, `NEGATIVE_CACHE_TTL_SECS`,
  `STALE_IF_ERROR_GRACE_SECS`, `MAX_CACHE_TTL_SECS`).
- `backend/src/services/proxy_service.rs` — revalidation, stale-if-error and
  negative caching on the buffered and streaming paths.
- Cache-correctness invariants: #1611, #3749 (410 negative-cached like 404).
- #4028 — the yank asymmetry documented above.
