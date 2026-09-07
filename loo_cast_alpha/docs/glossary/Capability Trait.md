---
canonical_name: Capability Trait
status: WIP-draft
aliases:
  - Capability Type Trait
  - Capability Tag
---

A Capability Trait is a dynamic, Rhai-compatible capability-side interface/marker/contract that a
[[Capability Type]] can implement.
It replaces the older template/tag/profile pressure in active doctrine.

The intuition is close to a Rust trait, but adapted to Vapor's dynamic capability graph, Rhai declaration surface, and
runtime validation model.

Core rules:

- A Capability Trait may define trait callbacks, not generic capability functions.
- A Capability Type may implement multiple Capability Traits.
- [[Capability Extension Slot]]s use trait bounds to decide which Capability Types or Capability Instances are valid
  extension/dependency candidates.
- A Capability Trait is summarized by a [[Capability Trait Signature]] for compatibility, validation, fingerprinting, and
  diagnostics.

Boundary:
Capability Traits are explicit semantic contracts.
Plain folder nesting, file placement, or source organization does not create trait implementation by itself.
Inheritance should not be a primitive in the capability model; inheritance-like behavior should be expressed through
trait implementation, composition, explicit extension slots, or other graph edges.

See also:

- [[Capability Type]]
- [[Capability Trait Signature]]
- [[Capability Extension Slot]]
- [[Capability Callback]]
- [[Capability Type Signature]]

#glossary
