---
canonical_name: Observer-Relative Simulation
status: draft
aliases: []
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
`active+above` captures an important USF direction, but it is no longer precise enough by itself.
The active scale is the first-class change-authority scale.
Higher scales remain simulated through summarized/scaled-time semantics rather than merely paused context.
Below-active detail is normally not broad active simulation, but it may exist through scoped sampling, scoped
simulation, or temporary inspection.
Lower-scale significance may overflow upward through explicit reconciliation, with an arithmetic-carry-like intuition.

Open pressure:
The short name for this policy is unresolved.
`active+above+scoped_below` is a pressure phrase, not final terminology.

Implementation-facing notes:

- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)

#glossary
