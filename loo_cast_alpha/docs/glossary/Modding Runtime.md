---
canonical_name: Modding Runtime
status: WIP-draft
aliases: []
---

The Modding Runtime is the runtime orchestration layer for mod loading, dependency resolution, registration, validation,
and lifecycle execution.
It enforces composition-time graph resolution before runtime lock.
For core architecture layers, additive composition is the default: mods can add definitions and integrations but should
not mutate or remove existing registered definitions after resolution.
It supports introducing new contract families and implementations through declared integration points.
The runtime realizes rules defined by the [[Modding Contract]] and composes with sibling families through
the [[Contract]].
Runtime lifecycle staging can be delegated to the [[Workflow Framework]].

User-facing order:
User-selected load order should not be a normal conflict-resolution mechanism.
Ordering should be graph/topology/policy-derived whenever possible.
The launcher should expose package/game/modpack choices and projected authoring surfaces, not arbitrary internal file or
folder rearrangement inside mods.

Implementation-facing notes:

- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)

#glossary
