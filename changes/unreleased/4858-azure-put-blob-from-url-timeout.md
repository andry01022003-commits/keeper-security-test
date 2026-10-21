---
section: Fixed
issues: [#4858]
---
- **Pushing an OCI blob of about 2 GiB to an Azure-backed repository no longer fails with 500 "Azure Put Blob From URL failed: error sending request"** (#4858). Every OCI blob finalize copies the uploaded bytes to their content-addressed key, and on Azure that copy is a synchronous Put Blob From URL for any source up to 5000 MiB. Azure sends no response until the copy is done, but the request went out on the data-plane client, whose 30-second read timeout aborted any copy that took longer while Azure was still copying. Monolithic and chunked uploads both end in that copy. Put Blob From URL now has a client of its own with no read timeout and a per-request deadline sized to the source: 60 seconds plus the time to copy it at 8 MiB/s. The source SAS now also outlives that deadline. Every other Azure request keeps the 30-second read timeout.
