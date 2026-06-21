---
canonical_name: Capability Slot Type
status: WIP-draft
aliases:
  - CapabilitySlotType
---

A Capability Slot Type is the current preferred name for the projected/gated slot/context shape.
Older profile/type-template labels are not active glossary pages anymore.

It describes what kind of capability surface can be attached, projected, requested, validated, or exposed in a specific
context.
Cardinality should not be baked directly into the slot type.
Cardinality should be modeled as a separate field, mode, or implementation detail on a slot using that slot type.

Boundary:
This term is owner-answer-informed but still WIP.
It should replace older profile/type-template wording in active design language where that wording means
projected/gated capability slot/context shape.

Open pressure:
A [[Capability Declaration]] may define a new Capability Slot Type only by explicit opt-in.
Do not assume every capability declaration automatically creates a slot type.
This captures the current pressure that capabilities, declarations, and slot types are tightly interrelated while still
keeping slot-type creation explicit.
[[Callback Type]], [[Callback Context Type]], and [[Callback Signature]] remain different callback-side concepts and are
not simply the same thing as Capability Slot Type.

#glossary
