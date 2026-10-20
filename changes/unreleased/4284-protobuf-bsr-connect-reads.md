---
section: Added
issues: [#4284]
---
- **Remote and virtual Protobuf repositories reverse-proxy BSR module metadata reads** (#4284). `POST /proto/{repo}/{Service}/{Method}` forwards an explicit allowlist of read-only `buf.registry.module.v1` Connect RPCs that have no local handler (for example `ModuleService/ListModules` and `LabelService/ListLabels`) to the remote repository's configured upstream, or to the readable remote members of a virtual repository in priority order. Only Connect headers are forwarded, so Artifact Keeper credentials never reach the upstream; responses carry only Connect headers, an upstream error is answered with a Connect error rather than the upstream body, and responses are capped at 16 MiB and 60 seconds and are not cached. `Download` is not proxied by this route. These calls still need a write-capable token, as every `POST` under `/proto` does today.
