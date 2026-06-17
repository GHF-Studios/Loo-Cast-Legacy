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

- `core_engine`, `core_mod`, and `base_mod` are reserved role names and literal required Rust crate names for those
  roles.
- Every [[Vapor Product Instance Stack]] must resolve these roles.
- A custom [[Engine]] is constituted by its own `core_engine` plus matching `core_mod`.
- A custom [[Game]] is constituted by its own `base_mod`.
- The role is mandatory, but the artifact filling the role can be replaced by selecting another valid implementation.
- `core_engine` and `core_mod` are bundled together as the Engine fixture; independent `core_mod` replacement is
  forbidden for Phase 3.
- `base_mod` is the literal Game crate/artifact and must declare the Engine/core_mod identity and version constraints it
  expects.

Naming analogy:
These names are special in roughly the same way Rust treats `main` or `lib` as special entry/module targets.
The name signals that the artifact is not just another plain module; it occupies a reserved structural role.

Other mods may depend on or extend the selected reserved roles, but they do not replace the need for the reserved roles
to exist.

Current code mismatch:
The current `loo_cast_alpha` crate layout may still contain transitional/sandbox crate names.
This glossary entry captures owner-confirmed Phase 3 direction, not a claim that the current alpha workspace already
satisfies the required `core_engine`, `core_mod`, and `base_mod` crate-name shape.

#glossary
