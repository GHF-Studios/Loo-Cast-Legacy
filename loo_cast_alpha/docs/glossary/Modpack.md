---
canonical_name: Modpack
status: WIP-draft
aliases:
  - modpack
  - Engine Modpack
  - Game Modpack
  - Mod Modpack
---

A Modpack is a user/modpack-author-facing composition object for selecting compatible mods and nested modpacks inside
the [[Vapor Ecosystem]].

Current owner-answer-informed framing:

- Modpacks should be typed by attachment target: Engine Modpack, Game Modpack, and recursive Mod/Extension Modpack are
  current pressure categories.
- A modpack may depend on another modpack only when the target [[Enginepack]], [[Gamepack]], version, and fingerprint
  constraints are compatible.
- Nested modpacks must not be conceptually flattened away; they are useful separation and organization boundaries even
  when validation eventually resolves one coherent capability graph.
- Nested/wrapped composition is mostly bottom-up, although an initial top-down initialization/discovery pass may still
  exist.
- A resolved stack should preserve enough path/fingerprint hierarchy to explain which Engine, Game, mods, nested
  modpacks, versions, and exposed user-facing capability paths are present.
- The complete selected launch composition is a [[Packagepack]].
- Modpacks should be publishable as their own [[Steam Workshop]] items and may depend on other Workshop items.
- Nested modpacks should be represented through both Workshop dependencies and Vapor metadata dependencies where useful.
- [[Vapor.toml]] is the current pressure term for modpack/composition manifest data.

Boundary:
The unqualified word `package` is currently too overloaded for core planning.
Prefer [[Packagepack]], [[Enginepack]], [[Gamepack]], Modpack, Steam package, [[Source Artifact]], [[Build Artifact]], or
[[Distributable Artifact]] depending on the concrete meaning.

Open pressure:
Nested folder or pack organization does not create implicit defaults.
[[Vapor.toml]] should describe each folder/artifact root explicitly, while capability visibility and dependencies remain
normal explicit graph relationships.

Phase 3 lock-candidate anchor:
Modpack Phase 3 behavior is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W02, P3-W07, P3-W08, and P3-W11.
Phase 3 must prove nested Modpacks, Workshop-backed Modpack dependencies, explicit dependency/conflict validation, and
visible contribution to Packagepack fingerprints/outputs where applicable.

See also:

- [[Enginepack]]
- [[Gamepack]]
- [[Packagepack]]
- [[Extension Mod]]
- [[Capability Graph Diagnostics]]
- [[Modding Runtime]]

#glossary
