---
canonical_name: Fingerprint
status: WIP-draft
aliases:
  - Vapor Fingerprint
---

A Fingerprint is a deterministic identity/compatibility summary for a [[Vapor Ecosystem]] object or subgraph.
It is stronger and richer than a plain ID/path/hash because it can combine stable machine-readable structure,
human-inspectable manifest information, hashes, versions, dependency shape, and public capability paths.

Current owner-answer-informed uses:

- compatibility checks
- diagnostics
- caching
- publishing
- reproducibility
- downloaded Workshop content verification
- packagepack, modpack, enginepack, gamepack, mod, and scoped sub-node summaries

A [[Packagepack]] fingerprint should be able to include selected [[Enginepack]], selected [[Gamepack]], [[Modpack]]s,
mods, versions, Workshop IDs, local dev IDs, public [[Capability]] paths, and [[Rhai Asset]] declaration identities.
Nested modpack boundaries should remain visible in at least some fingerprint projections.

Boundary:
A fingerprint may include both content-addressed hashes and a semantic manifest-like representation.
Not every consumer needs the same projection; player-facing, modpack-author-facing, and developer-facing views may show
different detail.
Failed fingerprint validation is mode-dependent:
Developer Mode should require an explicit unsafe flag to launch despite mismatch.
Pack Author Mode may allow it only behind an explicit unsafe setting.
Player Mode should block launch.

Phase 3 lock-candidate anchor:
The Phase 3 fingerprint scope is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W02, P3-W09, and P3-W11.
Phase 3 fingerprints must support deterministic composition comparison, Packagepack output, downloaded Workshop content
verification, diagnostics, caching, publishing, and reproducibility.
This is an integrity/compatibility/reproducibility rail, not a claim that Vapor provides a hostile-code sandbox in
Phase 3.

#glossary
