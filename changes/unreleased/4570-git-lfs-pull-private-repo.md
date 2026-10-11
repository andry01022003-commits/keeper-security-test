---
section: Fixed
issues: [#4570]
---
- **`git lfs pull` from a private Git LFS repository no longer hangs** (Closes #4570). The batch API marked every download action `"authenticated": true` but gave it an empty `header`. git-lfs trusts an authenticated action and sends the object GET with the action's headers only, so the GET carried no credentials, got a 401, and git-lfs retried forever. Download actions now carry the caller's `Authorization` header, as upload and verify actions already did, and an action is marked `authenticated` only when it carries one. Batch responses are sent with `Cache-Control: no-store`, since they can now echo a credential.
