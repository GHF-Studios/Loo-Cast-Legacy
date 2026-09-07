---
canonical_name: Gamepack
status: WIP-draft
aliases:
  - gamepack
  - Game Pack
---

A Gamepack is the user/modpack-author-facing composition object that selects one [[Game]]/`base_mod` artifact and its
Game-attached contributions inside the [[Vapor Ecosystem]].

Current owner-answer-informed framing:

- A running Vapor instance has exactly one selected [[Game]] from the selected Gamepack.
- The selected Game is the literal `base_mod` crate/artifact for that stack.
- `base_mod` must declare the required [[Engine]]/`core_mod` identity and version constraints.
- Game-attached mods and nested Game-targeting modpacks extend the effective Game composition.
- The effective Game composition produces identity/fingerprint information that downstream modpack and
  [[Extension Mod]] compatibility checks must respect.
- Local/downloaded Gamepack artifacts may group actual game-side contributions, while published Workshop items may
  reference dependencies rather than embedding every contribution literally.

Boundary:
The Gamepack depends on the selected [[Enginepack]].
The full launch composition containing the Gamepack is a [[Packagepack]].
The Gamepack does not make the [[USF]] a product slot; USF remains a public/API-facing subsystem inside the
[[Spacetime Engine]].

Phase 3 lock-candidate anchor:
Gamepack Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W08 and P3-W09.
Phase 3 must prove the default Game fixture, an alternative Game fixture, Game Mods, Extension Mods, and Game-side
contribution to the hello-world-on-steroids output/fingerprint without introducing gameplay/worldmodel semantics.

See also:

- [[Enginepack]]
- [[Modpack]]
- [[Packagepack]]
- [[Reserved Built-In Mod Role]]
- [[Vapor Product Instance Stack]]

#glossary
