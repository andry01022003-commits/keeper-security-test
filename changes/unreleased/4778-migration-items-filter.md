---
section: Fixed
issues: [#4778]
---
- **`GET /api/v1/migrations/{id}/items` applies its `status` and `item_type` filters, and the migration report says how many failed items it left out** (#4778). Both query parameters were accepted and ignored, so `?status=failed` paged through every item of the job and the pagination total counted all of them; they now narrow the page and the total. Items that share a `started_at` (every item not yet started) are ordered by id as well, so pages no longer repeat or skip rows. The report still lists at most 100 failed items, and its `summary.failed_items_omitted` now counts the rest; `?status=failed` on the items endpoint lists them all.
