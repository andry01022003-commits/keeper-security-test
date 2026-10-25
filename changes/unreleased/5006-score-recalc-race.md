---
section: Fixed
issues: [#5006]
---
- **A repository's security score no longer goes stale when two of its scans finish at the same time** (#5006). Each score recalculation read the repository in its own snapshot with no coordination, so when two scans completed together one recalculation could store a score computed before the other scan had committed, and the other failed with a serialization error that nothing retried. The score, `grade_status` and coverage then kept counting the finished scan as missing until the next scan or delete in that repository (on 1.11 replicas this could show as a false F). Recalculations of one repository now run one at a time under a database advisory lock and are retried on a serialization failure, so the stored score always includes every completed scan. This covers every path that recalculates a score: scan completion, artifact deletes, finding triage and SBOM ingest.
