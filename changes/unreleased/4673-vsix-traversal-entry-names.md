---
section: Security
issues: [#4673]
---
- **A `.vsix` whose archive entries carry traversal or absolute paths is refused at publish** (#4673). The server only inspects a VSIX (bounded manifest reads, #3961) and never extracts it, but it stored and served such an archive verbatim, so any client that unzipped it naively could be made to write outside its target directory (`../../etc/passwd`, `/tmp/x`). Publishing now walks the zip's central directory, names only, and refuses an entry with a `..` segment or an absolute, backslash-rooted or drive-qualified name with `400 VALIDATION_ERROR`, before anything is stored. This applies to any zip body on the publish route, including one published under the legacy `x-*` headers.
