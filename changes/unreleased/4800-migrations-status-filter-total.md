---
section: Fixed
issues: [#4800]
---
- **The migration jobs list reports `pagination.total` for the rows its `status` filter matched** (#4800). `GET /api/v1/migrations?status=failed` already narrowed the rows on the page, but the `COUNT(*)` behind `pagination.total` was computed without the filter, so a filtered view reported every job the caller could see and `total_pages` — the page count and "N of M" figure the UI derives from it — was too large. The count now carries the same filter as the listing; unfiltered totals are unchanged.
