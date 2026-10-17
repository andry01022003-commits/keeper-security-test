---
section: Added
issues: [#4711]
---
- **Conda attestation, install-script and allowlist controls can now be graphed and alerted on** (#4711). Three Prometheus counters, all labelled by repository key and never by package name: `ak_conda_attestation_total{repository,outcome}` counts CEP-27 attestation decisions (`verified`, `rejected` when verification is required and the bundle does not verify, `failed` when it is stored under the explicit `CONDA_ATTESTATION_REQUIRE_VERIFIED=false` opt-out, and `absent` for each hosted package upload, which carries no attestation of its own); `ak_conda_install_script_findings_total{repository,kind}` counts the `pre-link`, `post-link` and `pre-unlink` scripts found in uploaded packages; and `ak_conda_allowlist_dropped_total{repository,member}` counts the remote records a virtual channel's package allowlist kept out of a merge, per remote member, once per merge. Documented, with example queries, in `docs/operations/conda-security-metrics.md`.
