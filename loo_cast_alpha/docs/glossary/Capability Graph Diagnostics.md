---
canonical_name: Capability Graph Diagnostics
status: WIP-draft
aliases:
  - Graph Diagnostics
  - Mod Conflict Diagnostics
---

Capability Graph Diagnostics are reports produced from invalid or suspicious [[Capability]]/slot graph states.

Current framing:

- The low-level primitive is an invalid capability/slot graph state.
- `mod conflict` is still a valid user-facing diagnosis.
- Explicit mod-wide conflicts are author-friendly metadata layered over graph validation.
- Validation errors should first be classified by graph primitive, then projected into player, modpack-author, and
  developer diagnostics.
- Developer diagnostics should expose graph terms, product terms, concrete paths, and validation details.
- Player-facing diagnostics should name conflicting mods, modpacks, packagepacks, enginepacks, gamepacks, or qualified
  package artifacts and explain practical next actions without requiring capability graph expertise.

Phase 3 pressure:
Phase 3 acceptance should be a matrix of multiple valid and invalid [[Modpack]] configurations rather than one canonical
command.
At minimum, the matrix should include an unmodded/default stack, a modded first-party stack, a simple non-first-party
Engine stack, a simple non-first-party Game stack, a nested modpack stack, and an invalid/conflict stack, plus useful
permutations.
Successful runs should produce composition/fingerprint artifacts and launcher-visible logs/output.
Invalid runs should produce launcher-native diagnostics before `core_engine` starts.
The first validation proofs should include cycle detection, explicit conflict detection, missing required providers,
duplicate singleton providers, and visibility violations as the relevant type/cardinality model permits.

#glossary
