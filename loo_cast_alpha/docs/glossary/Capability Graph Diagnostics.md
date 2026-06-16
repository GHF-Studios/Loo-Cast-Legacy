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
- Developer diagnostics should expose graph terms, product terms, concrete paths, and validation details.
- Player-facing diagnostics should name conflicting packages/mods and explain practical next actions without requiring
  capability graph expertise.

Phase 3 pressure:
The first validation proofs should include cycle detection and explicit conflict detection.
Missing required provider and duplicate singleton provider are likely important too, but they depend on the still-open
capability type-template/cardinality model.

#glossary
