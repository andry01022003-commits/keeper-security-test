---
section: Security
issues: [#4780]
---
- **The backend image's bundled `buildctl` is rebuilt from BuildKit v0.33.1, which vendors containerd v2.3.6 and clears three MEDIUM containerd advisories** (#4780). The push-time Trivy scan of the 1.11.2 UBI image reported CVE-2026-53493 (fixed in containerd 2.3.6), CVE-2026-53495 and CVE-2026-95838 (both fixed in 2.3.5) against `/usr/local/bin/buildctl`, whose BuildKit v0.33.0 source vendors `github.com/containerd/containerd/v2` v2.3.4. BuildKit's own v0.33.1 patch release on the same line already vendors v2.3.6, so the `buildkit-bin` stage moves its pinned tag and commit (`8c91502c`) instead of carrying a containerd override; the go1.27.2 toolchain floor and the `golang.org/x/net` v0.60.0 override stay as they were. `buildctl --version` now reports v0.33.1. Nothing is added to `.trivyignore`. The Alpine image does not ship `buildctl` and is unchanged.
