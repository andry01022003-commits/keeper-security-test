---
section: Fixed
issues: [#4335]
---
- **Conan recipe and package `/latest` report the revision's stored time** (#4335). Both endpoints built their `time` from the clock at request time, so the same revision showed a new time on every read, and that time did not match the revision's entry in `/revisions`. `/latest` now returns the first entry of the same revision-history query `/revisions` uses, with the same time, so repeated reads are stable and agree with the history. For a remote repository answering from its upstream, the upstream's own `time` is passed through.
