---
canonical_name: Enginepack
status: WIP-draft
aliases:
  - enginepack
  - Engine Pack
---

An Enginepack is the user/modpack-author-facing composition object that selects one [[Engine]] fixture and its
Engine-attached contributions inside the [[Vapor Ecosystem]].

Current owner-answer-informed framing:

- A running Vapor instance has exactly one selected Engine fixture from the selected Enginepack.
- The selected Engine fixture is a coupled `core_engine` plus matching `core_mod` pair.
- Engine-attached mods and nested Engine-targeting modpacks extend the effective Engine composition.
- The effective Engine composition produces identity/fingerprint information that downstream [[Gamepack]] compatibility
  checks must respect.
- Enginepack compatibility should be SemVer-like where version constraints are involved, but exact fingerprint semantics
  remain under pressure.
- Local/downloaded Enginepack artifacts may group actual engine-side contributions, while published Workshop items may
  reference dependencies rather than embedding every contribution literally.

Boundary:
An Enginepack is not the same as an unqualified package.
Use qualified terms such as Steam package, [[Source Artifact]], [[Build Artifact]], or [[Distributable Artifact]] when
those meanings are intended.
The full launch composition containing the Enginepack is a [[Packagepack]].

Phase 3 lock-candidate anchor:
Enginepack Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W08 and P3-W09.
Phase 3 must prove the default Engine fixture, an alternative Engine fixture, Engine Mods, and Engine-side contribution
to the hello-world-on-steroids output/fingerprint without introducing worldmodel semantics.

See also:

- [[Gamepack]]
- [[Modpack]]
- [[Packagepack]]
- [[Reserved Built-In Mod Role]]
- [[Vapor Product Instance Stack]]

#glossary
