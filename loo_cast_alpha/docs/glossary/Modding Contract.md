---
canonical_name: Modding Contract
status: WIP-draft
aliases:
  - Modding Contract Surface
source_of_truth: []
---

The Modding Contract defines mod declaration, life-cycle, dependency, compatibility, replacement, composition, and
integration boundaries.
Mods may introduce new capabilities and new contract families, plus their implementations.
This [[Contract Family]] composes with other contract families through the [[Contract]], while runtime orchestration is
handled by the [[Modding Runtime]].

Current pressure:
`additive-only` must be scoped carefully.
Inside a selected locked runtime composition, definitions should not be secretly mutated after resolution.
Across composition-time selection, slot ownership, capability-node policy, and ecosystem-level replacement, the system
must allow non-additive shapes such as exclusive replacement, variadic extension, ordered registries, optional providers,
and explicit integration apertures.

Therefore, `mod conflict` should usually be treated as a symptom of an invalid capability/slot graph rather than the
primary primitive.

#glossary
