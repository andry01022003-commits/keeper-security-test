---
section: Fixed
issues: [#4334]
---
- **Conan package search on a local repository returns each binary's configuration, so `conan list -p` and `conan download -p` filter correctly** (#4334). The hosted package search answered every package id with an empty object, so a settings filter such as `-p "os=Linux"` matched no binary: `conan list` reported none, and `conan download` exited 0 having downloaded nothing. Each package id now carries its stored `conaninfo.txt` as `{"content": ...}`, the shape conan_server returns and the Conan 2 client parses, for local repositories and for the local members of a virtual. `?list_only=True` still answers with the ids alone, without reading the files.
