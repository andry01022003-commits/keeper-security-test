---
section: Fixed
issues: [#4818]
---
- **Members of the `repository-owner` role pass the repository `admin` check again** (#4818). Since 1.11.0 the repository administration routes (email subscriptions, the staging release target, upstream auth, egress proxy, routing rules, upstream filter, conda allowlist, PyPI tracks, the age gate and per-repository security settings) answered 403 to a `repository-owner` member, including the user who created the repository, because the check read fine-grained permission rules only and ignored role assignments. It now uses the same resolver as the write and delete checks, so a role carrying `admin` on the repository is honoured; `developer` and `reader` members are still refused.
