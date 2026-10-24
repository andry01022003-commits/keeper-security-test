---
section: Added
issues: [#4363]
---
- **An instance administrator can lock a repository's proxy scan policy so a repository administrator cannot weaken it** (#4363). Mirroring the age-gate lock, any `PUT /api/v1/repositories/{key}/security` by an instance administrator now locks the proxy scan policy (migration 300, `scan_configs.proxy_scan_instance_locked`) unless the body sends `"instance_locked": false`. While locked, a repository administrator may still tighten the policy but gets `403` for turning `scan_on_proxy` off, moving `proxy_scan_action` towards `record_only`, or loosening the severity gate, and for sending `instance_locked` at all. The lock is shown as `instance_locked` on the security config GET and on `GET /api/v1/security/configs`. Existing configurations start unlocked. During a rolling upgrade, a replica still on 1.11 does not know the lock and accepts a repository administrator's write; finish the upgrade before relying on it. See docs/security/scan-on-proxy.md.
