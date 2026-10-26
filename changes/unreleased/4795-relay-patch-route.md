---
section: Added
issues: [#4795]
---
- **The remote-instance relay accepts `PATCH`, so the web UI can forward partial updates to a remote instance** (#4795). The relay handled GET, POST, PUT and DELETE only, so repository settings edits and chunked-upload session updates relayed from the UI were answered with `405 Method Not Allowed`; PATCH now shares POST/PUT's auth checks, path validation, query-string forwarding and content-type/body handling, and its route is documented in the OpenAPI spec.
