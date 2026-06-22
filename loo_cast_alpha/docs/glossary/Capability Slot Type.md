---
canonical_name: Capability Slot Type
status: legacy-bridge
aliases:
  - CapabilitySlotType
---

Capability Slot Type is legacy bridge terminology for the concept now better expressed as
[[Capability Extension Slot]] plus [[Capability Trait]] bounds and signature metadata.

Older notes used Capability Slot Type for several overlapping ideas:

- profile/type-template identity
- graph edge or slot kind
- accepted capability kind
- projected declaration context shape
- Rust-side validation/materialization authority

Active wording splits those apart:

- [[Capability Type]] names the capability kind/category.
- [[Capability Trait]] names explicit interface/marker/contract bounds.
- [[Capability Type Signature]] and [[Capability Trait Signature]] describe metadata/fingerprint-like surfaces.
- [[Capability Extension Slot]] names the explicit graph extension/dependency point.
- [[Callback Type]], [[Callback Context Type]], and [[Callback Signature]] remain callback-side concepts.

Boundary:
Do not use Capability Slot Type for new doctrine unless explicitly discussing legacy notes.
Use Capability Extension Slot when the intended meaning is "this graph position accepts candidates satisfying these
trait bounds and slot policies."
Cardinality, ordering, optionality, replacement behavior, and conflict behavior should stay explicit slot policy rather
than trait identity.

Open pressure:
Old pages that say Capability Slot Type probably need case-by-case migration.
Some usages should become Capability Extension Slot.
Some should become Capability Trait.
Some should become Callback Type, Callback Context Type, Capability Projection API, or host-contract wiring.

See also:

- [[Capability Extension Slot]]
- [[Capability Trait]]
- [[Capability Type Signature]]
- [[Capability Trait Signature]]
- [[Callback Type]]

#glossary
