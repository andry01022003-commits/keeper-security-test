---
section: Fixed
issues: [#4933]
---
- **LDAP web login keeps an administrator's deactivation of an account** (#4933). A successful directory bind on the web LDAP login used to set a deactivated account active again. LDAP accounts now keep `is_active` across a federated login, the login of a deactivated account is refused, and an administrator must reactivate the account explicitly.
