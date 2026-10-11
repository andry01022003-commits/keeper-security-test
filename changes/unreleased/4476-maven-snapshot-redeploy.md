---
section: Fixed
issues: [#4476]
---
- **Redeploying a Maven snapshot artifact to the same timestamped path is accepted again instead of failing with 409** (#4476). A build that deploys the same attached artifact twice in one `mvn deploy` uploads the same timestamped file (`object-1.0-20261006.124037-3-clear.jar`) twice. Up to 1.8.0 the second upload replaced the first; since #3839 (1.10.0) the upload path treated a resolved unique snapshot as immutable and refused it, which failed the build. Snapshots are documented as overwritable, so every file under a `-SNAPSHOT` version directory, unique or not, is republished in place again. Release coordinates still answer 409, a version that only contains the token (`1.0-SNAPSHOT-rc1`) is still a release, and the proxy cache's classification of unique snapshots is unchanged.
