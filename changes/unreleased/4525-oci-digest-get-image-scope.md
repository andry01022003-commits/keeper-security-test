---
section: Fixed
issues: [#4525]
---
- **`GET` / `HEAD /v2/<name>/manifests/<digest>` on a hosted repository now resolve the digest under `<name>`, not repository-wide** (#4525). After `DELETE <repo>/xa@<digest>` (scoped to the image name since #4466), the same digest was still served as `xa@<digest>` while another image name in the repository (`xb:v1`) tagged it. On Local and Staging repositories a digest reference now resolves through a tag (or by-digest push) of the requested image name, or, for a manifest that is no longer tagged under it (an overwritten tag, #1681), through the repository's manifest records unless every manifest row that image name had for that digest is in the trash. A digest deleted under one image name therefore answers `404 MANIFEST_UNKNOWN` under that name and keeps resolving under the others. `DELETE` by digest uses the same resolution, so repeating a delete under the deleted name is a `404`. Remote and Virtual repositories are unchanged.
