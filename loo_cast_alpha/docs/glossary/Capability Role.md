---
canonical_name: Capability Role
status: WIP-draft
aliases: []
---

A Capability Role describes the fundamental archetypal contract shape of a capability under the [[Capability Contract]].
This role is hard-coded as contract metadata and each capability has exactly one role.
Current roles are `input` and `output`.
Input capabilities gather or ingest data without directly mutating canonical game-world state, while output capabilities
present or display state without directly mutating canonical game-world state.

Current pressure:
`input` and `output` are not sufficient as a final authority model.
Potential additional role surfaces include authority, reconciler, realizer, bridge, and mutator, but these are not
settled.
Canonical mutation authority is outside capability objects in reconcile/commit/apply, while capabilities relay requests
and expose structured authority/API surfaces.
Until mutation routing is explicitly modeled, role terminology should not pretend canonical state transitions are fully
explained.
Runtime authority-split implications and possible role-surface expansion are tracked here:
[Capability Role and State Authority Notes](Capability%20Role%20and%20State%20Authority%20Notes.md)

#glossary
