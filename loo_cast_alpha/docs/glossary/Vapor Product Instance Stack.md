---
canonical_name: Vapor Product Instance Stack
status: WIP-draft
aliases:
  - Vapor product stack
  - Vapor product instance
---

A Vapor Product Instance Stack is the concrete selected launch/composition stack for a [[Vapor Ecosystem]] product
instance.

Current framing:

- `core_engine`, `core_mod`, and `base_mod` are [[Reserved Built-In Mod Role]] names and non-negotiable architecture
  pillars.
- The stack must have some `core_engine`, some matching `core_mod`, and some `base_mod`.
- Mandatory does not mean globally irreplaceable.
- `core_engine` and `core_mod` are coupled and should be replaced as a compatible Engine pair.
- `base_mod` is mandatory but replaceable as the [[Game]] layer.
- Additional mods, submods, extension slots, registries, and integration apertures hang off the selected pillars rather
  than replacing the need for those pillars.

This term is meant to prevent the common confusion between "this slot/pillar is required" and "this exact first-party
artifact can never be replaced."

#glossary
