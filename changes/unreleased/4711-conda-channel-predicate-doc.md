---
section: Fixed
issues: [#4711]
---
- **The conda channel policy predicates are documented as failing closed, which is what they do** (#4711). The `CondaPolicyPredicates` documentation, and the OpenAPI description generated from it, said that `conda.allowed_channels` and `conda.denied_channels` fall back to the owning repository's key when a package records no channel of origin. That fallback never existed: since 1.11.0 an unknown channel is an unknown origin, so a non-empty `allowed_channels` refuses the package and `denied_channels` cannot match it. Only the documentation changes; a test now pins the documentation and the evaluation together.
