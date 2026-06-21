---
canonical_name: Vapor Product Stack
status: WIP-draft
aliases:
  - Vapor product stack
---

Vapor Product Stack is the current name for the high-level Vapor product/composition relationship.

Current owner-answer-informed framing:

- [[Vapor Ecosystem]] is the Steam-exclusive SDK, launcher, distribution, identity, modding, composition, validation,
  [[Capability]], and [[Rhai]] substrate.
- [[Engine]] and [[Game]] are generic Vapor roles.
- A concrete launched composition selects exactly one Engine fixture and exactly one Game fixture.
- [[Packagepack]] is the player/modpack-author-facing launch composition object that brings selected [[Enginepack]],
  [[Gamepack]], [[Modpack]], [[Engine Mod]], [[Game Mod]], and [[Extension Mod]] contributions together.
- Concrete first-party engine/game names are examples of this stack, not the definition of the stack.

Boundary:
This is not a USF product stack.
USF-like internals are engine subsystem details when a concrete Engine chooses to expose them.
The product-stack concept should stay engine/game-independent except when a specific example is needed.

See also:

- [[Vapor Ecosystem]]
- [[Packagepack]]
- [[Enginepack]]
- [[Gamepack]]
- [[Modpack]]
- [[Vapor Launcher]]

#glossary
