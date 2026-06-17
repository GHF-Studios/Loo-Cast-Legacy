---
canonical_name: Game
status: WIP-draft
aliases:
  - Vapor Game
---

A Game is a [[Vapor Ecosystem]]-level product/composition concept and one mandatory pillar of a
[[Vapor Product Instance Stack]].
The [[Loo Cast]] Game is a first-party Game instance.

Current framing:

- A Game is represented by a mandatory singleton `base_mod` layer in a Vapor product stack.
- `base_mod` is the [[Reserved Built-In Mod Role]] name and literal required Rust crate/artifact name for any Vapor
  Game.
- `base_mod` is mandatory but replaceable as a Game-level Vapor concept.
- `base_mod` is dynamically linked/loaded relative to the Engine fixture, even though a Game slot is required for a
  meaningful playable product.
- `base_mod` must declare the required [[Engine]]/`core_mod` identity and version constraints.
- The selected Game composition is mediated through a [[Gamepack]].
- A Game may later expose its own mod slots or extension apertures, but that is not part of the immediate baseline.

The Game concept is not the same as a packaged mod artifact by itself.
It is the active playable/content layer mounted onto an [[Engine]] through Vapor composition rules.

#glossary
