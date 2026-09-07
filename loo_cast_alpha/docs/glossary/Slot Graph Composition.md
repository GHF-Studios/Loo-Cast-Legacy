---
canonical_name: Slot Graph Composition
status: WIP-draft
aliases: []
---

Slot Graph Composition defines composition-time ownership and extension structure as declared extension-slot filling
from a root through nested capability/mod/framework graphs.
Slots are an abstract helper concept and are not limited to whole mods.
They may exist at engine, game, mod, capability-node, module, or smaller API-surface levels.
At the current abstraction level, a slot is best understood as a node-owned [[Capability Extension Slot]] that accepts
candidate capability nodes whose [[Capability Type Signature]] satisfies required [[Capability Trait]] bounds under a
cardinality/policy shape.
This is stronger than "edge with policy" but still keeps room for richer edge metadata.

In the first-party [[Vapor Product Instance Stack]], the `core_engine` and matching `core_mod` form a coupled mandatory
Engine pillar, and the `base_mod`/[[Loo Cast]]-type layer fills the mandatory Game pillar.
These are [[Reserved Built-In Mod Role]] names before they are ordinary extension slots.
Future gameplay/content extension slots are possible, but not required for the immediate baseline.
`core_mod` is mandatory but replaceable only as part of selecting another valid coupled Engine pair.
`base_mod` is mandatory but replaceable as the Game-level Vapor concept.
The correction is therefore mandatory matched occupancy, not global irreplaceability.
For Phase 3, the selected Engine pair is mediated by [[Enginepack]], the selected Game by [[Gamepack]], and additional
contributions by compatible [[Modpack]]s, [[Engine Mod]]s, [[Game Mod]]s, and [[Extension Mod]]s.

Slot mechanics:

- A slot is owned by a parent node.
- A filled slot is itself a [[Capability]] node in that parent/child relation.
- A slot should accept candidates through explicit trait bounds and signature validation, not through implicit folder
  position or one hardcoded concrete type.
- Multiple required traits are allowed, with exact bound-composition rules still open.
- Cardinality belongs to the slot policy: exactly one, zero-or-one, zero-or-more, one-or-more, exact N, tuple-like,
  struct-like, enum-like, registry-like, or other Rust-inspired shapes remain possible.
- A node cannot have itself as an extension-slot candidate, dependency, or dependant.
- Cycles are invalid, but deep acyclic nesting is allowed when justified.

Composition is valid only when required slots resolve and singleton-critical ownership resolves to exactly one owner per
scope key under the [[Modding Contract]].
Invalid graphs hard-fail before runtime and are guaranteed to not be the case once the [[Runtime Lock]] is reached.

Timing:
Slot filling and graph composition happen at composition time during ultra-early runtime, before the runtime lock.
Runtime graph mutation should be forbidden by default.
If runtime dynamism is needed, model it through explicit capability/runtime-substrate policy, for example a registry
capability backed by a [[Capability Kernel]], rather than as arbitrary post-lock slot mutation.

Open policy vocabulary includes exclusive slots, variadic slots, ordered registries, optional providers, and integration
apertures.
These are not final field-level schema yet; they are current pressure terms for preventing `slot` from collapsing into
one overly rigid mechanism.
Replacement currently means selecting a different capability satisfying the same extension-slot bounds before lock.

#glossary
