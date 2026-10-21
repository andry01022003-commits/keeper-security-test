---
section: Fixed
issues: [#4831]
---
- **Logout now revokes the session's refresh token even when the refresh token is not in the request** (#4831). `POST /api/v1/auth/logout` revoked the refresh-token family only when the request carried the refresh token, in the body or the `ak_refresh_token` cookie. A bearer logout with an empty body, and a browser logout from the web UI, carry neither, so logout answered 200 while the session's refresh token kept minting new access tokens until it expired. Access tokens minted with a refresh token now carry that session's family id, and logout revokes the presenting session's family whether or not the refresh token is included. Other sessions of the same user stay signed in, as before. Access tokens issued before upgrading carry no family id; a logout with one of them behaves as before until the session next refreshes.
