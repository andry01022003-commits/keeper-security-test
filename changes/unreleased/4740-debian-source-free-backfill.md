---
section: Fixed
issues: [#4740]
---
- **A Debian repository imported before 1.11.2 can recover its package registrations without the source system** (#4740). Re-running the migration job was the only documented repair for a repository whose `.deb` rows had no `debian` control document, catalog entry, or maintainer-script analysis, which needs access to the source. A new admin backfill walks the live `pool/**/*.deb` rows that lack that metadata, reads each `control.tar` member from the repository's own storage with bounded ranged requests, and records the same control document, catalog row, and script analysis a native upload writes. It also soft-deletes the migration-origin `dists/` index copies from the artifact list, since AK renders and signs its own, while leaving hosted rows alone. The run is idempotent, bounded per call, and reports unreadable or inconsistent packages without aborting the rest.
