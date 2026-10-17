---
section: Changed
issues: [#4754]
---
- **The Sponsors recognition section is generated from the active GitHub sponsorships** (#4754). `scripts/release/sponsors.sh` prints the `### Sponsors` block for `CHANGELOG.md` (`--level 2` for the release notes) from the organization's active sponsorships, naming public sponsors with their GitHub handle and only counting sponsors who chose a private sponsorship; `--check FILE` fails when a changelog or release-notes file omits a public sponsor. The 1.11.1 section, release notes and README, which named two of the seven active sponsors, are brought up to date in the same change.
