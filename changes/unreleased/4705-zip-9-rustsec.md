---
section: Security
issues: [#4705]
---
- **`zip` 8.6.0 -> 9.0.4 and the rattler crates to the 0.57 line for RUSTSEC-2026-0367, -0368 and -0369** (#4705). The advisories cover `ZipArchive::extract()`: SUID/SGID bits preserved on extraction, symlinks created with targets outside the destination directory, and Unix permissions applied through symlinks. The backend never calls `extract()` (it reads archive entries by index and writes with `ZipWriter`), so no shipped behaviour was affected and none changes; `rattler_package_streaming` moves to 0.27.15 and `rattler_conda_types` to 0.57.1 because the newer streaming crate is the one built on zip 9. The one pre-9 instance left in the lockfile, zip 2.4.2, is a build-time-only dependency of `utoipa-swagger-ui` 8.1.0 that unpacks the vendored swagger-ui distribution during the build and is not linked into the binary; it is recorded in `.cargo/audit.toml` with that rationale until utoipa-swagger-ui builds with zip 9.
