---
canonical_name: Engine
status: WIP-draft
aliases:
  - Vapor Engine
---

An Engine is a [[Vapor Ecosystem]]-level product/composition concept.
The [[Spacetime Engine]] is a first-party Engine instance.

Current framing:

- An Engine is defined through a `core_engine` plus its matching `core_mod` fixture.
- The matching `core_mod` is hard-required for that Engine and should not be treated as freely removable,
  deselectable, or mix-and-match replaceable.
- A non-Spacetime Engine may exist inside Vapor, including one that exposes little or no end-user modding.
- An Engine may ignore most of the recommended [[Capability]] model after the required bootstrap/entrypoint surface, but
  that is a workaround path rather than the intended ergonomic path.

The Engine concept is broader than the first-party Spacetime implementation, but the active first-party docs should not
use this breadth to make the [[USF]] look directly replaceable as a product-level slot.

#glossary
