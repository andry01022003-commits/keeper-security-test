---
section: Fixed
issues: [#4333]
---
- **`conan remove` works against a local Conan repository** (#4333). The Conan router registered no DELETE routes, so removing a recipe or a binary that had just been uploaded and installed answered 404, which the client reports as `Recipe not found` / `Binary package not found`, and the package stayed. The four routes the Conan 2 client deletes are now served: a whole recipe, a recipe revision (with its binaries), every binary of a recipe revision, and a single package revision. Removal needs the `delete:artifacts` token scope and the repository's `delete` permission, is refused on remote (405) and virtual (400) repositories, and answers 404 when nothing matched. Removed files are soft-deleted like every other delete path, so storage GC reclaims them.
