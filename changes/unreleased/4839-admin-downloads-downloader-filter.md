---
section: Added
issues: [#4839]
---
- **`GET /api/v1/admin/downloads` takes a `downloader=all|anonymous|authenticated` filter** (#4839). Anonymous downloads are recorded with a null `user_id`, which the UUID-typed `user_id` filter could not select, so an admin reviewing unauthenticated traffic before turning off `allow_anonymous_download` had to page through every download. `downloader=anonymous` returns only the rows without a user and `downloader=authenticated` only the attributed ones, on hosted and proxy rows alike; the filter combines with the artifact, repository, IP, source and date filters, also applies to `/downloads/by-ip/{ip}` and `/downloads/by-user/{user_id}`, and is honoured by anything that pages the listing (the web export). `downloader=anonymous` together with a `user_id` is rejected with 400. Omitted, the listing is unchanged.
