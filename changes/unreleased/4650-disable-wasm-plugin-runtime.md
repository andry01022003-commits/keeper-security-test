---
section: Added
issues: [#4650]
---
- **Operators can disable the WASM plugin runtime completely with `PLUGINS_ENABLED=false`** (#4650). The backend then skips plugin registry and Wasmtime construction, does not load stored plugins at startup, and leaves the plugin management, format-handler, and `/ext` protocol routes unregistered. The setting defaults to enabled, preserving existing deployments, and is independent of `PLUGINS_REQUIRE_SIGNED`, which continues to control plugin admission when the runtime is active.
