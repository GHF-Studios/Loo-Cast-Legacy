---
canonical_name: Capability-Centric Semantics
status: WIP-draft
aliases: []
---

Capability-Centric Semantics means project meaning and authority are modeled through capabilities first, while
execution still runs on lower-level runtime primitives in the [[Runtime Substrate]].
Capability contracts define semantic authority; substrate primitives define operational execution.
This is capability-first, not capability-exclusive.

Global rudimentary surfaces (for example math and logging) may exist as explicitly defined global capability
surfaces, without collapsing domain-specific capability boundaries.
Global scope at this layer still remains bounded by the [[Capability Graph Scope Envelope]] and script-safe projection
policy.

See also:

- [[Capability]]
- [[Capability Graph Scope Envelope]]
- [[Runtime Substrate]]
- [[Global Capability Surface]]
- [[Script Safety]]

#glossary
