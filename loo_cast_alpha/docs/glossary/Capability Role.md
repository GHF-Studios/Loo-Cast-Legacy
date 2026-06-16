---
canonical_name: Capability Role
status: WIP-draft
aliases: []
---

A Capability Role is an unresolved pressure term for describing how a capability participates in graph semantics.
Older `input` and `output` wording is likely misleading if it implies there are only two node roles.

Current pressure:
`input` and `output` may be better understood as dependency/dependant directions or slot-facing roles rather than
complete capability roles.
Potential additional role surfaces include authority, reconciler, realizer, bridge, and mutator, but these are not
settled.
Canonical mutation authority is outside capability objects in reconcile/commit/apply, while capabilities relay requests
and expose structured authority/API surfaces.
Until mutation routing is explicitly modeled, role terminology should not pretend canonical state transitions are fully
explained.
Runtime authority-split implications and possible role-surface expansion are tracked here:
[Capability Role and State Authority Notes](Capability%20Role%20and%20State%20Authority%20Notes.md)

#glossary
