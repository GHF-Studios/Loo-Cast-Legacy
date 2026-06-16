---
canonical_name: Vapor Ecosystem
status: WIP-draft
aliases:
  - Vapor
  - Vapor Modding Ecosystem
  - Vapor SDK
---

The Vapor Ecosystem is the current name for the [[Steam]]-exclusive ecosystem/[[SDK]] layer above concrete [[Engine]]s,
[[Game]]s, mods, and modpacks.
It is the layer through which engines, games, mods, modpacks, tooling, identity, distribution, runtime protocol
constraints, contracts/traits, and [[Steam Workshop]] integration are developed, maintained, composed, validated, and
distributed.

At this layer, the [[Spacetime Engine]] is the first-party Engine instance, and [[Loo Cast]] is the first-party Game
instance built through that stack.
Other engines, games, and mods are intended to be possible in the same broad ecosystem, including cases that replace the
Spacetime Engine or do not expose the same modding surface.
Such engines may ignore most of the recommended [[Capability]] model after the required bootstrap/entrypoint surface,
but doing so is a workaround/hack path rather than the intended ergonomic route.

The Vapor Ecosystem is specifically designed around [[Steam]] as its external substrate for identity, distribution,
authorization, and [[Steam Workshop]]-like capabilities.
It is not a generic wrapper over arbitrary storefronts or distribution platforms.

Product boundary:
Vapor is product-like in its own right, but it is expected to be available through ownership of the [[Loo Cast]] product
bundle rather than as an unrelated standalone public product.

SDK boundary:
The Vapor SDK is the development surface for everything in the stack: engines, games, mods, modpacks, and related
tooling.
Individual engines and games may add to that developer experience, but development should happen either in the root
source repository or through the Vapor SDK.

Contract boundary:
Vapor includes runtime protocol constraints, contracts/traits, package/composition structure, and the high-level
Engine/Game/mod/modpack concepts.
This is why "Vapor Modding Ecosystem" is a useful alias but too narrow as the only name.

Authority note:
This name and framing are owner-answer-informed and intentionally WIP.
It should not be treated as a fully finalized brand, contract, or implementation boundary yet.

#glossary
