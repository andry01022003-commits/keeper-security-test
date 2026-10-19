---
section: Fixed
issues: [#4781]
---
- **A hung `/readyz` dependency now costs the 2 s deadline once per cache window, not on every probe** (#4781). `/readyz` bounded each dependency twice with independent 2 s timers: an outer one started before the cache lock was taken and an inner one started after. Any lock wait let the outer timer fire first and drop the probe before it stored its timed-out result, so the next `/readyz` probed the hung OpenSearch, scanner, LDAP or storage backend again for the full deadline. Both timers now share one deadline, with the outer (lock-wait) one 100 ms later, so the timeout is always cached.
