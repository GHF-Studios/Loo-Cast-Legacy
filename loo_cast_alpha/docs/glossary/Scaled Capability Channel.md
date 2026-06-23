---
canonical_name: Scaled Capability Channel
status: WIP-draft
aliases: []
---

A Scaled Capability Channel is a USF-compatible, scale-scoped execution face of a capability implementation.
It is not part of the general definition of [[Capability]].
General capabilities may be unscaled, globally scoped, or scoped by some non-USF policy.
When a capability participates in USF scale semantics, channel availability across scales is governed by
[[Scale Support]], and execution binds to active [[Scale Slice]] context.
Declaration scripts access channel-relevant capability objects through type/trait/callback-tailored `ctx`
capability-object subgraphs, and runtime [[Capability Instance]]s invoke sanctioned callback logic through those
resolved channels.
Those subgraphs are derived from hierarchical API graph composition (atomic + composite nodes) with include/exclude
path declarations.
It is governed through the capability contract context inside the [[Contract]] and the USF scale contract, without being
its own standalone contract family.

#glossary
