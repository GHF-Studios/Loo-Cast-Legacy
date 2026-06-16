---
canonical_name: Scale Realizer
status: WIP-draft
aliases:
  - Phenomena Realizer
---

A Scale Realizer is the semantic realization capability bound to one [[Scale Slice]].
It was formerly referred to as a "phenomena realizer", renamed to keep scale-centric terminology coherent.
Each active scale slice must resolve to exactly one effective scale realizer as constrained by
[[Scale Realizer Cardinality]].

Current pressure:
The term remains useful because each scale still needs exactly one effective realizer, but the name is under pressure.
[[Phenomenon]] implementations appear to own the concrete materialization and re-aggregation logic that realizes detail
within a scale.
Do not use `Scale Realizer` wording to imply that chunk internals or phenomena-owned detail logic are separate
authorities from the phenomenon model without a dedicated pass.

#glossary
