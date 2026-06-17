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
The launcher should expose [[Packagepack]], [[Enginepack]], [[Gamepack]], [[Modpack]], and projected authoring surfaces,
not arbitrary internal file or folder rearrangement inside mods.

Composition staging:

1. Discover the artifact graph.
2. Build the user/modpack-author projection.
3. Perform shallow metadata pre-validation from direct `conflicts`, `dependencies`, and similar composition metadata.
4. Expand the deeper dependency/capability graph.
5. Perform deep validation against the full developer-facing graph.
6. Establish the [[Runtime Lock]].
7. Hand off to the locked runtime graph.

The launcher lifecycle is separate from the engine startup lifecycle.
Invalid modpack/config states should fail in launcher-native diagnostics before `core_engine` starts.
After validation succeeds, engine/runtime logs should surface through the launcher console and may also be written to a
log file.

Implementation-facing notes:

- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)

#glossary
