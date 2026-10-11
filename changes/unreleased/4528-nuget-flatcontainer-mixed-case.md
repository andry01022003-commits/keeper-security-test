---
section: Fixed
issues: [#4528]
---
- **A NuGet Remote's flat-container download no longer 404s when the client spells the file name in mixed case** (#4528). The proxy lowercased only the package id segment of the upstream flat-container path and forwarded the version and the `.nupkg` file name as sent, so `GET .../flatcontainer/Newtonsoft.Json/13.0.3/Newtonsoft.Json.13.0.3.nupkg` asked nuget.org for a path it does not serve. The NuGet V3 flat container defines the whole path as lowercase, so the id, the version and the file name are now all lowercased: every spelling reaches the same upstream URL and the same cache entry, and a scan-blocked version answers 403 under any spelling. Real NuGet clients already send lowercase and see no change.
