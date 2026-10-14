---
section: Fixed
issues: [#4672]
---
- **Starting an OCI blob upload with an over-limit `Content-Length` is refused with 413 at once** (#4672). `POST /v2/<name>/blobs/uploads/`, both the session-create form with a body and the monolithic `?digest=` form, streamed the body until it crossed `MAX_UPLOAD_SIZE` before answering 413, and a client that declared more than it sent was only reaped by the 408 stall watchdog; over HTTP/2 through a gateway the client could see a `PROTOCOL_ERROR` stream reset instead of a status. The declared length is now checked before anything is registered or a body byte is read, exactly as `PATCH` already did, so the 413 is answered before the request stream is reset.
