---
section: Fixed
issues: [#4884]
---
- **`PUT /api/v1/peers/{id}/network-profile` now routes to the peer network-profile handler instead of returning a 500 with a raw axum extractor error** (#4884). The router had been mounted at `/peers/network-profile` without the `:id` segment; it now requires the peer ID, rejects malformed IDs with a normal 400, and returns 404 for an unknown peer.
