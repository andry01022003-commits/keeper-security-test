---
section: Fixed
issues: [#4865]
---
- **Deleting a versioned Generic or MLModel repository now removes the objects of its older revisions as well** (#4865). The delete collected storage keys only from the repository's live `artifacts` rows. Older revisions live in `artifact_versions`, which no `artifacts` row names, and those rows cascade away with the repository, so the history objects stayed on filesystem and object storage with nothing left to derive them from, and the storage GC orphan scan, which starts from `artifacts`, could never find them. The key set now unions the repository's live artifacts and its own revisions under the same location-scoped sharing guard, and a revision key the delete spares because another repository still names it is handed to the storage-GC candidate set so the next GC pass reclaims it once no reference remains.
