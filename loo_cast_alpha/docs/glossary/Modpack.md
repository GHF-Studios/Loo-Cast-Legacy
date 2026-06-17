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
- A resolved stack should preserve enough path/fingerprint hierarchy to explain which Engine, Game, mods, nested
  modpacks, versions, and exposed user-facing capability paths are present.
- The complete selected launch composition is a [[Packagepack]].

Boundary:
The unqualified word `package` is currently too overloaded for core planning.
Prefer [[Packagepack]], [[Enginepack]], [[Gamepack]], Modpack, Steam package, build artifact, source package, or runtime
artifact depending on the concrete meaning.

See also:

- [[Enginepack]]
- [[Gamepack]]
- [[Packagepack]]
- [[Extension Mod]]
- [[Capability Graph Diagnostics]]
- [[Modding Runtime]]

#glossary
