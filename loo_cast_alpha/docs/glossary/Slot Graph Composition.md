---
canonical_name: Slot Graph Composition
status: WIP-draft
aliases: []
source_of_truth: []
---

The Slot Graph Composition defines composition-time ownership and extension structure as declared slot filling from a
root through nested capability/mod/framework graphs.
Slots are an abstract helper concept and are not limited to whole mods.
They may exist at engine, game, mod, capability-node, module, or smaller API-surface levels.

In the first-party stack, the `core_engine`-type layer exposes required structure filled by a `core_mod`/[[USF]]-type
layer, and the `base_mod`/[[Loo Cast]]-type layer fills gameplay/content structure.
Future gameplay/content extension slots are possible, but not required for the immediate baseline.

Composition is valid only when required slots resolve and singleton-critical ownership resolves to exactly one owner per
scope key under the [[Modding Contract]].
Invalid graphs hard-fail before runtime and are guaranteed to not be the case once the [[Runtime Lock]] is reached.

Open policy vocabulary includes exclusive slots, variadic slots, ordered registries, optional providers, and integration
apertures.
These are not final field-level schema yet; they are current pressure terms for preventing `slot` from collapsing into
one overly rigid mechanism.

#glossary
