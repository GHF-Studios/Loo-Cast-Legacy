---
canonical_name: Capability Type Template
status: superseded-pressure
aliases: [ ]
---

Capability Type Template is an older pressure term for a Rust-side template/category concept.
Current batch-005 follow-up pressure says this mostly means the same thing as [[Capability Slot Type]], but with worse
wording.

It should not be used as settled schema terminology until the [[Capability Slot Type]] model is clarified.

The useful remaining idea is that Rust contracts/traits and registration wiring constrain how [[Capability Declaration]]s
are validated and materialized.

Current pressure:
[[Capability Slot Type]] may need to cover built-in graph shape and cardinality forms such as singleton, exact-count,
zero-or-more, one-or-more, tuple-like, struct-like, enum-like, registry-like, and other Rust-inspired composition
shapes.
These primitives may themselves be compatible or incompatible with each other.
Do not treat the current labels `exclusive`, `variadic`, `ordered registry`, `optional provider`, or `integration
aperture` as settled schema until this template layer is clarified.

#glossary
