---
section: Fixed
issues: [#4302]
---
- **Incus chunked image uploads work behind more than one backend replica** (#4302). A chunked upload appended its chunks to a file on the local disk of the replica that served the start, so a chunk or the completion routed to another replica found nothing and the upload failed. Each chunk is now staged as an object on the repository's storage backend and recorded on the session in commit order, so any replica can take any chunk; the completing replica copies the chunks into its own staging file and finalizes as before. Chunk objects are deleted when the session completes, fails or is cancelled, and the hourly upload-staging sweep reclaims those of an abandoned session.

  Limits: the completion reads the whole image back from the storage backend before hashing it, so it takes one extra read of the image and the completing replica's staging disk must still hold it; each chunk costs one extra object write. A session started before the upgrade keeps the local-file path and still needs session affinity to finish. Moving the staging itself off local disk is a separate change.
