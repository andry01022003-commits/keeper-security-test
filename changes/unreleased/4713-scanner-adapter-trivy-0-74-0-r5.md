---
section: Security
issues: [#4713]
---
- **The scanner adapter's bundled trivy is rebuilt as 0.74.0-r5 with containerd 2.3.4 for CVE-2026-95837; published as scanner-adapter 1.4.2** (#4713). The HIGH advisory (containerd's CRI checkpoint restore bypassing the destination security context) was published against the containerd 2.3.3 that trivy 0.74.0 links for its containerd image source, and it failed the image publish gate on every build from 2026-10-17 without a byte of this repository changing. Trivy itself runs no CRI plugin, so no scan behaviour changes; the rebuild clears the gate. Upstream trivy fixes it by moving to containerd 2.4.1 in 0.75.0 and 0.76.0, which the adapter cannot take alone because it requires the trivy server sidecar to report the same version as the bundled client; the coordinated move of both to 0.76.0 is #4712.
