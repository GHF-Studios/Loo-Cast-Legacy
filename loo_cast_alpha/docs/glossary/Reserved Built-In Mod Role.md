---
canonical_name: Reserved Built-In Mod Role
status: WIP-draft
aliases:
  - Reserved mod role
  - Built-in critical mod
  - Special mod role
---

A Reserved Built-In Mod Role is a [[Vapor Ecosystem]] role/name that marks a mod-like artifact as structurally special
instead of a plain ordinary module or optional extension.

Current framing:

- `core_engine`, `core_mod`, and `base_mod` are reserved role names.
- Every [[Vapor Product Instance Stack]] must resolve these roles.
- A custom [[Engine]] is constituted by its own `core_engine` plus matching `core_mod`.
- A custom [[Game]] is constituted by its own `base_mod`.
- The role is mandatory, but the artifact filling the role can be replaced by selecting another valid implementation.

Naming analogy:
These names are special in roughly the same way Rust treats `main` or `lib` as special entry/module targets.
The name signals that the artifact is not just another plain module; it occupies a reserved structural role.

Other mods may depend on or extend the selected reserved roles, but they do not replace the need for the reserved roles
to exist.

#glossary
