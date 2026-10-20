---
section: Fixed
issues: [#4816]
---
- **NuGet V2 upstream translation no longer drops XML entities and text before them in description/authors** (#4816). When parsing V2 OData feeds, character/entity references (`&amp;`, `&#xF6;`) and CDATA sections were treated as separate events that overwrote field values; the parser now accumulates text across `Text`, `GeneralRef`, and `CData` events before committing to the field.
