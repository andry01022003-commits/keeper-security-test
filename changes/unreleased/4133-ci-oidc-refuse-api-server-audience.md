---
section: Security
issues: [#4133]
---
- **A CI OIDC provider can no longer be given a Kubernetes API server's audience** (#4133). A provider whose `audience` is `https://kubernetes.default.svc` or `https://kubernetes.default.svc.cluster.local` accepts every pod's default ServiceAccount token, and a token presented to Artifact Keeper is then also valid against the API server. Creating or updating a provider with such an audience now fails with 400 naming it, for every provider type, and nothing is written. Providers that already carry one are kept, and named in a `security` warning at each startup so they can be changed.
