---
section: Fixed
issues: [#4379]
---
- **`ansible-galaxy collection install` resolves collections through a Virtual Ansible repository** (#4379). The Galaxy metadata routes built their documents from the repository's own artifacts, which a Virtual never has, so `versions/` answered an empty list and the collection and version detail routes answered 404 while `download/` already served the members' tarballs; the install failed with "Could not satisfy the following requirements". On a Virtual these routes now read the members the caller may read, in priority order: the version list and the collection list are the members' union, the detail documents come from the first member that has them (a Remote member answers from its upstream as before), and every `href` and `download_url` points back at the Virtual. A member's upstream outage is reported as that error rather than as an empty list when no other member has versions.
