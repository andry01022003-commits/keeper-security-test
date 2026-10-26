---
section: Fixed
issues: [#4857]
---
- **Removing the `LDAP_*` environment variables now disables the LDAP provider they created** (#4857). The `LDAP_*` bootstrap seeds an enabled LDAP provider, but a later boot without `LDAP_URL` / `LDAP_BASE_DN` left that provider enabled, so the login page kept offering only the LDAP form and local login stayed hidden for a directory the operator had removed. The boot now disables (never deletes) the provider the environment manages, matching what removing `OIDC_*` already does (#2819), and logs a warning naming it. Providers created through the admin API are never touched, and an administrator who re-enables the provider through the admin API takes ownership of it, so the next boot without `LDAP_*` leaves it enabled. Re-setting the variables does not re-enable a provider the boot disabled; re-enable it through the admin API.
