---
canonical_name: Capability Slot Type
status: WIP-draft
aliases:
  - CapabilitySlotType
---

A Capability Slot Type is a capability-graph slot/edge type: the declared shape of a slot a capability type/template
can require, import, expose, or implement.
Older profile/type-template labels are not active glossary pages anymore, but the distinction between a concrete
Capability Type Template, the slot type that accepts it, and the metadata signature describing either surface remains
real pressure.

It describes what kind of capability surface can be attached, projected, requested, validated, or exposed in a specific
context.
Owner direction now treats Capability Slot Types as graph edge types or edge-type-like declarations in the heterogeneous
capability graph.
Cardinality should not be baked directly into the slot type.
Cardinality should be modeled as a separate field, mode, or implementation detail on a slot using that slot type.

Boundary:
This term is owner-answer-informed but still WIP.
It should not be flattened into callback metadata, projection scope, or generic path policy.
Scope-envelope pages describe access/projection boundaries; Capability Slot Type names a more concrete slot/edge
compatibility surface.

Open pressure:
A Capability Type can declare or import one or more Capability Slot Types it wants to expose or fill.
Other [[Capability Instance]]s whose declarations/implementations satisfy those slot types may then occupy those slots.
[[Vapor.toml]] is expected to import/define slot-type metadata in a Cargo.toml-like role.
Local, non-imported slot-type implementations should be backed by Rust host contracts, ideally with macro-generated
wiring where practical.
Rhai declarations may provide concrete capability/callback/slot-type declaration payloads, but those asset classes need
a follow-up split before this is final.
Do not assume every capability declaration automatically creates a slot type.
[[Callback Type]], [[Callback Context Type]], and [[Callback Signature]] remain different callback-side concepts and are
not simply the same thing as Capability Slot Type.

#glossary
