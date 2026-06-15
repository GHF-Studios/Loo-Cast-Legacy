---
canonical_name: Observer-Relative Simulation
status: draft
aliases: []
source_of_truth: []
---

Observer-Relative Simulation defines view-conditioned detail resolution over the scale system.
Given an active [[Scale View]], simulation keeps required higher-scale context available while eliding lower-scale
detail
outside the active view unless explicitly bridged.
Interpretation and reconciliation of this behavior run through scale-aware capabilities in the [[Capability Runtime]]
with the goal of predictable cross-scale traversal semantics.
Concrete runtime techniques (for example chunk loading around a camera/player locus, [[Entity Proxy]], or
[[Portal Traversal Semantics]]) may realize this behavior but are not the defining concept at this glossary layer.

Current owner-answer-informed pressure:
active+above is a hard USF direction, but "never simulate below active" is a default policy rather than an absolute ban
on scoped, explicit detail sampling or localized inspection.
Only the active scale should normally emit first-class change authority.
Higher scales remain meaningful and can act as summarized/reactive authority over lower scales, while lower-scale
significance may overflow upward through explicit reconciliation.

Implementation-facing notes:

- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)

#glossary
