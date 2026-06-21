---
canonical_name: Packagepack
status: WIP-draft
aliases:
  - packagepack
---

A Packagepack is the complete user/modpack-author-facing launch composition in the [[Vapor Ecosystem]].
It is the spiritual successor to the ambiguous unqualified `package`, but with a deliberately narrower meaning.

Current owner-answer-informed framing:

- A Packagepack selects exactly one [[Enginepack]].
- A Packagepack selects exactly one [[Gamepack]] compatible with that Enginepack.
- A Packagepack includes the compatible [[Modpack]]s, [[Engine Mod]]s, [[Game Mod]]s, and [[Extension Mod]]s selected for
  that launch composition.
- A Packagepack carries or produces resolved compatibility/[[Fingerprint]] metadata for the selected stack.
- A Packagepack is the thing the launcher validates, summarizes, and launches as one configured Vapor instance.
- A Packagepack is also the object players launch, modpack authors compose, and developers create raw content artifacts
  for.
- Players can select and launch Packagepacks, but should not meaningfully create or edit them in Player Mode beyond
  surface-level launch options.
- Creation and structural editing belong to Modpack Author Mode and Developer Mode.
- A Packagepack may be public, installable, authorable, and publishable.
- Packagepacks should be able to be declared through Rhai/Vapor declaration surfaces, but hardcoded declarations are not
  structurally impossible.
- Enginepack/Gamepack/mod contents may be physically downloaded and grouped/aggregated into local artifacts, while
  packaged Workshop items may reference those dependencies rather than literally embedding every dependency.

Boundary:
A Packagepack is not the same as a Steam package, [[Source Artifact]], [[Build Artifact]], [[Distributable Artifact]],
or arbitrary archive.
Those terms must stay qualified because they describe source, build, or distribution containers rather than the selected
composition itself.
The scoped term `Packagepack` is allowed despite using `package`; the problematic term is unqualified `package`.

Manifest pressure:
[[Vapor.toml]] is the current pressure term for packagepack/composition manifest data.
Published versions of packagepacks must not depend on unpublished local artifacts.
Local workspace artifacts are acceptable for authoring/modpack-author workflows before publication.

Acceptance pressure:
Phase 3 should use a scenario/integration-test suite rather than a single canonical command or overly simplistic matrix.
The suite should center on sets of engines, games, mods, extension mods, engine mods, game mods, and packs that can be
mixed and matched manually to verify expected success/failure behavior.
[[Phase 3 Vapor Scenario Suite]] is the current anchor for this pressure.

See also:

- [[Enginepack]]
- [[Gamepack]]
- [[Modpack]]
- [[Vapor Product Instance Stack]]

#glossary
