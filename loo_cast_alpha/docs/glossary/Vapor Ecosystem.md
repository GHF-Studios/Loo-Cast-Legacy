---
canonical_name: Vapor Ecosystem
status: WIP-draft
aliases:
  - Vapor
  - Vapor SDK
---

The Vapor Ecosystem is the current name for the [[Steam]]-exclusive ecosystem/product layer above generic
[[Engine]]s, [[Game]]s, mods, [[Packagepack]]s, [[Modpack]]s, [[Enginepack]]s, and [[Gamepack]]s.
It is the layer through which engines, games, mods, packagepacks, modpacks, enginepacks, gamepacks, tooling, identity,
distribution, runtime protocol constraints, contracts/traits, and [[Steam Workshop]] integration are developed,
maintained, composed, validated, and distributed.
It includes the [[SDK]], [[Vapor Launcher]], distribution/identity rails, [[Capability]] substrate, [[Rhai]] authoring
substrate, and composition/validation model.

At this layer, the [[Spacetime Engine]] is the first-party Engine instance, and [[Loo Cast]] is the first-party Game
instance built through that stack.
Other engines, games, and mods are intended to be possible in the same broad ecosystem, including cases that replace the
Spacetime Engine or do not expose the same modding surface.
Such engines may ignore most of the recommended [[Capability]] model after the required bootstrap/entrypoint surface.
That path is allowed as an escape hatch, but Vapor's intended model is still capability/Rhai-first.

The Vapor Ecosystem is specifically designed around [[Steam]] as its external substrate for identity, distribution,
authorization, and [[Steam Workshop]]-like capabilities.
It is not a generic wrapper over arbitrary storefronts or distribution platforms.
Future storefront abstraction is explicitly deferred.
If it ever exists, it should likely be a separate [[steam-like-platform-contracts]] layer above or beside Vapor, not a
Phase 3 requirement.

Product boundary:
Vapor is product-like in its own right, but it is expected to be available through ownership of the [[Loo Cast]] product
bundle rather than as an unrelated standalone public product.
Current pressure also allows Vapor itself to become a public/open-source Rust crate/tooling stack, while first-party
engine/game products can remain proprietary until a later decision.
See [[Vapor Crate Topology]] for current implementation-facing crate/workspace pressure.

SDK/launcher boundary:
The Vapor SDK is the development surface for everything in the stack: engines, games, mods, modpacks, and related
tooling.
Individual engines and games may add to that developer experience, but development should happen either in the root
source repository or through the Vapor SDK.
The [[Vapor Launcher]] is the user/modpack-author-facing composition, validation, launch, diagnostics, and
Steam/Workshop surface.
Phase 3 owner direction requires real Steam authentication/identity, Workshop download/install, and Workshop
upload/publish before the phase can be considered complete, while still allowing local/offline authoring mode for
iteration.

Contract boundary:
Vapor includes runtime protocol constraints, contracts/traits, pack/composition structure, and the high-level
Engine/Game/mod/modpack concepts.
It also defines the foundational [[Capability]] and [[Rhai Asset]] authoring model before any concrete engine/game
extends it.
Modding-only names are too narrow and should not be treated as active aliases.

Metadata boundary:
[[Vapor.toml]] is now the active pressure term for package/mod/composition metadata such as dependencies, conflicts,
target roles, version constraints, and Steam/Workshop publication fields.
Rhai remains foundational for authored capability declarations, but Vapor.toml is allowed and expected where manifest
metadata is the better fit.

Phase 3 lock-candidate anchor:
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md) is the active Phase 3 execution anchor for
Vapor.
Phase 3 must prove the SDK, launcher, Steam/Workshop public distribution rail, [[Capability]]/[[Rhai Asset]] substrate,
[[Packagepack]] composition, diagnostics, fingerprints, local/offline authoring, and hello-world-on-steroids
Engine/Game fixtures.
Phase 3 must not implement USF/worldmodel/rendering/save-load/gameplay systems.
Glossary pages define terms; the execution spec decides what Phase 3 must prove.

Terminology boundary:
The unqualified word `package` is currently too overloaded for active planning.
Prefer scoped terms such as [[Packagepack]], [[Enginepack]], [[Gamepack]], [[Modpack]], Steam package,
[[Source Artifact]], [[Build Artifact]], or [[Distributable Artifact]].

Authority note:
This name and framing are owner-answer-informed and intentionally WIP.
It should not be treated as a fully finalized brand, contract, or implementation boundary yet.

#glossary
