---
canonical_name: Engine
status: WIP-draft
aliases:
  - Vapor Engine
---

An Engine is a [[Vapor Ecosystem]]-level product/composition concept and one mandatory pillar of a
[[Vapor Product Instance Stack]].
The [[Spacetime Engine]] is a first-party Engine instance.

Current framing:

- An Engine is defined through a coupled `core_engine` plus matching `core_mod` pair.
- `core_engine` and `core_mod` are [[Reserved Built-In Mod Role]] names and literal required Rust crate names for any
  Vapor Engine.
- A Vapor product instance must have some `core_engine` and some matching `core_mod`.
- `core_mod` is therefore mandatory, but not globally irreplaceable.
- Replacing `core_mod` means selecting another valid coupled Engine pair, not dropping it or freely mixing it with an
  unrelated `core_engine`.
- A custom Engine artifact should physically contain or colocate its `core_engine` executable and matching `core_mod`
  artifact.
- For Phase 3, independent `core_mod` replacement is forbidden; the Engine pair is selected through the [[Enginepack]].
- A non-Spacetime Engine may exist inside Vapor, including one that exposes little or no end-user modding.
- An Engine may ignore most of the recommended [[Capability]] model after the required bootstrap/entrypoint surface, but
  that is a workaround path rather than the intended ergonomic path.

The Engine concept is broader than the first-party Spacetime implementation, but the active first-party docs should not
use this breadth to make the [[USF]] look directly replaceable as a product-level slot.

#glossary
