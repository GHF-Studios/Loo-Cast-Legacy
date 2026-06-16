---
canonical_name: Capability Type Template
status: WIP-draft
aliases: [ ]
---

The Capability Type Template is the Rust-side template authority for one capability/profile category.
It is host-side contract/template authority and is not the same concept as [[Capability Profile]].

It is defined through Rust contracts/traits and registration wiring, and it constrains how capability declarations are
validated and materialized.

Scripts do not define templates; scripts define [[Capability Declaration]]s that target a capability type template.

At the current draft stage, the active template set is intentionally small and fixed (for example `Scale`, `Metric`,
`Phenomenon`, and `Scale Realizer`).

Current pressure:
Capability type templates may also need to cover built-in "types of types" for graph shape and cardinality, such as
singleton, exact-count, zero-or-more, one-or-more, tuple-like, struct-like, enum-like, registry-like, and other
Rust-inspired composition shapes.
These template primitives may themselves be compatible or incompatible with each other.
Do not treat the current labels `exclusive`, `variadic`, `ordered registry`, `optional provider`, or `integration
aperture` as settled schema until this template layer is clarified.

#glossary
