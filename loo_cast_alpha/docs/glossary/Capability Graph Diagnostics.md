---
canonical_name: Capability Graph Diagnostics
status: WIP-draft
aliases:
  - Graph Diagnostics
  - Mod Conflict Diagnostics
---

Capability Graph Diagnostics are reports produced from invalid or suspicious [[Capability]] /
[[Capability Extension Slot]] graph states.

Current framing:

- The low-level primitive is an invalid capability, trait, callback, signature, or extension-slot graph state.
- `mod conflict` is still a valid user-facing diagnosis.
- Explicit mod-wide conflicts are author-friendly metadata layered over graph validation.
- Validation errors should first be classified by graph primitive, then projected into player, modpack-author, and
  developer diagnostics.
- Developer diagnostics should expose graph terms, product terms, concrete paths, and validation details.
- Player-facing diagnostics should name conflicting mods, modpacks, packagepacks, enginepacks, gamepacks, or qualified
  package artifacts and explain practical next actions without requiring capability graph expertise.
- Player, modpack-author, and developer diagnostics are projections of richer underlying diagnostics.
- Developer diagnostics should include raw file paths, Steam IDs, fingerprints, graph paths, Rhai declaration paths, and
  validation primitive names where relevant.
- Steam/Workshop/network failures should be recoverable diagnostics rather than panics.
- Internal invariant violations in Vapor tooling may panic-fast during development builds.
- Workshop content is trusted as user-installed content, but still validated for integrity, compatibility, and
  fingerprint correctness.
- Vapor does not currently sandbox Workshop content as hostile code.
- Steam identity, Steam ownership checks for upload, and Vapor fingerprints are the Phase 3 trust rails.

Phase 3 lock-candidate direction:
Phase 3 acceptance should be a testing suite of multiple valid and invalid [[Packagepack]] and [[Modpack]]
configurations rather than one canonical command, one CI-only integration-test suite, or an overly simplistic matrix.
At minimum, the suite should include an unmodded/default stack, a modded first-party stack, a simple non-first-party
Engine stack, a simple non-first-party Game stack, a nested modpack stack, and invalid/conflict stacks, plus useful
permutations.
Successful runs should produce composition/fingerprint artifacts and launcher-visible logs/output.
Invalid runs should produce launcher-native diagnostics before `core_engine` starts.
The first validation proofs should include cycle detection, explicit conflict detection, missing required providers,
duplicate singleton providers, version mismatches, invalid target roles, bad typed Rhai declarations, fingerprint
mismatches, corrupted/incomplete Workshop downloads, and visibility violations as the relevant type/trait/cardinality
model permits.
Declared conflicts should be checked as path+version predicates.
Vapor should also attempt deeper scans for conflicts that should have been declared but were not, without trying to
prove that every declared conflict is warranted by exact content comparison.
See [[Phase 3 Vapor Testing Suite]] and
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W10 and P3-W11.

#glossary
