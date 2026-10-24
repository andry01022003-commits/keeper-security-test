---
section: Added
issues: [#2024]
---
- **Lifecycle policies take a `match.format` scope** (#2024). `"match": {"format": "docker"}` (or an array such as `["npm", "yarn"]`) limits a policy to repositories of those formats: the policy is skipped entirely in a repository of any other format, in the dry-run preview, the live run and the proxy-cache eviction alike, so one global policy can clean up build images without touching Maven or npm repositories. It combines with `match.path_prefix`, `match.version_pattern`, `exclude` and `min_keep`. Format names are the repository format keys (`docker`, `helm_oci`, `conda_native`, ...); an unknown name is rejected with 400 when the policy is created or updated. A server older than this release refuses to run a policy that carries `match.format` rather than ignoring the scope.
