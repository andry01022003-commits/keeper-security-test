---
section: Security
issues: [#4827]
---
- **SAML responses are read with entity references, character references and CDATA decoded, so the NameID, attributes and groups AK consumes are exactly the values the IdP signed** (#4827, GHSA-cp5x-hq47-4mg3). A Response that uses an entity other than the five predefined ones, or that carries a DOCTYPE, is refused, and whitespace inside a value is kept as signed rather than trimmed.
