---
canonical_name: Modding Contract
status: WIP-draft
aliases:
  - Modding Contract Surface
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

Current owner-answer-informed boundary:
At the highest core-architecture level, modding an existing `core_engine`, `core_mod`, or `base_mod` should be
additive-only by default.
Third-party mods may expose their own internal mutation/replacement semantics, but those semantics still resolve at
composition time under locked policy.

Diagnostics:
`mod conflict` remains a useful user-facing diagnosis, but it should usually be a report constructed from lower-level
invalid capability/slot graph facts.
Explicit mod-wide conflicts are author-friendly metadata layered over graph validation, not a replacement for graph
validation.
Developer diagnostics should expose graph/product details; player-facing diagnostics should explain the conflicting
mods, modpacks, enginepacks, gamepacks, or qualified package artifacts and likely actions without requiring graph
expertise.

Pack vocabulary:
For current active planning, avoid unqualified `package`.
Use [[Packagepack]], [[Enginepack]], [[Gamepack]], [[Modpack]], Steam package, [[Source Artifact]], [[Build Artifact]], or
[[Distributable Artifact]].

#glossary
