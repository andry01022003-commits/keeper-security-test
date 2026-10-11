---
section: Fixed
issues: [#4507]
---
- **Curation version constraints and version ordering now honour Debian/RPM and PEP 440 epochs** (Closes #4507). `version_compare` split versions on `.` and `-` and left an epoch glued to the first segment, so `1:2.0-1` compared as lower than `10.0-1` and `1:1.0` as lower than `2.0`. It now parses a leading `N:` (Debian/RPM) or `N!` (PEP 440 / conda) epoch, treats a missing epoch as 0, compares epochs numerically first and the rest in the existing natural order. This changes curation rule matches for epoch-bearing deb and rpm versions: a block rule `< 10.0` no longer matches `1:2.0-1`, and an allow rule `>= 10.0` now does. Listings ordered by this comparator (Hex, NuGet, the packages API) order epoch-bearing versions the same way.
