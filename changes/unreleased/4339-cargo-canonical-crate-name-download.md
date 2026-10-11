---
section: Fixed
issues: [#4339]
---
- **A Cargo pull-through now downloads crates whose canonical name has an uppercase letter, such as `Inflector`** (#4339). The Remote and Virtual download paths built the upstream `.crate` URL from the lowercased crate name, and crates.io stopped serving non-canonical spellings in December 2023, so every such crate answered 404 and `cargo fetch` failed on the first one. The upstream is now asked for the name exactly as the client requested it (cargo always uses the canonical spelling from the index), both for a registry's `dl` template (`{crate}` and `{prefix}`) and for the plain `api/v1/crates/{name}/{version}/download` path; the proxy cache key, the catalog and the index lookups stay lowercase, so every spelling shares one cache entry.
