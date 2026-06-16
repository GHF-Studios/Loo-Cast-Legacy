---
canonical_name: Slot Graph Composition
status: WIP-draft
aliases: []
---

The Slot Graph Composition defines composition-time ownership and extension structure as declared slot filling from a
root through nested capability/mod/framework graphs.
Slots are an abstract helper concept and are not limited to whole mods.
They may exist at engine, game, mod, capability-node, module, or smaller API-surface levels.
At the current abstraction level, a slot can be treated roughly as a [[Capability]] graph edge with policy, while
preserving room for richer policy shapes.

In the first-party [[Vapor Product Instance Stack]], the `core_engine` and matching `core_mod` form a coupled mandatory
Engine pillar, and the `base_mod`/[[Loo Cast]]-type layer fills the mandatory Game pillar.
These are [[Reserved Built-In Mod Role]] names before they are ordinary extension slots.
Future gameplay/content extension slots are possible, but not required for the immediate baseline.
`core_mod` is mandatory but replaceable only as part of selecting another valid coupled Engine pair.
`base_mod` is mandatory but replaceable as the Game-level Vapor concept.
The correction is therefore mandatory matched occupancy, not global irreplaceability.

Composition is valid only when required slots resolve and singleton-critical ownership resolves to exactly one owner per
scope key under the [[Modding Contract]].
Invalid graphs hard-fail before runtime and are guaranteed to not be the case once the [[Runtime Lock]] is reached.

Open policy vocabulary includes exclusive slots, variadic slots, ordered registries, optional providers, and integration
apertures.
These are not final field-level schema yet; they are current pressure terms for preventing `slot` from collapsing into
one overly rigid mechanism.

#glossary
