---
section: Fixed
issues: [#4907]
---
- **The PyPI age gate no longer blocks old releases whose filename spells the version differently from the Warehouse release** (#4907). The gate looked up a distribution's publish time by the exact version string in its filename, but Warehouse keys `releases` by the release's own spelling, so `cuda_toolkit-13.0.3.0-py2.py3-none-any.whl` (release `13.0.3`, uploaded months earlier) found no publish time and was withheld with a 451 indefinitely, which broke `pip install torch==2.14.1` on Linux. The lookup now falls back to PEP 440 equality on the download path and in both simple-index listing filters; a version that is not PEP 440-equal to a release still has no publish time and stays gated.
