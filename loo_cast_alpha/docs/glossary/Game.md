---
canonical_name: Game
status: WIP-draft
aliases:
  - Vapor Game
---

A Game is a [[Vapor Ecosystem]]-level product/composition concept.
The [[Loo Cast]] Game is a first-party Game instance.

Current framing:

- A Game is represented by a required singleton `base_mod` layer in the first-party stack.
- `base_mod` is replaceable as a Game-level Vapor concept.
- `base_mod` is dynamically linked/loaded relative to the Engine fixture, even though a Game slot is required for a
  meaningful playable product.
- A Game may later expose its own mod slots or extension apertures, but that is not part of the immediate baseline.

The Game concept is not the same as a packaged mod artifact by itself.
It is the active playable/content layer mounted onto an [[Engine]] through Vapor composition rules.

#glossary
