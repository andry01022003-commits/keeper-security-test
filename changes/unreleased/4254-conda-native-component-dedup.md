---
section: Fixed
issues: [#4254]
---
- **`conda_native` packages get the same scanner-component dedup as `conda` packages** (closes #4254). `ComponentDedup` only built a dedup scope when the repository format was exactly `conda`, so a package in a `conda_native` repository did not fold its scanner components against the PyPI distribution it vendors and could report the same component twice. The check now uses the shared conda-format test (`conda` or `conda_native`), as publisher trust already does.
