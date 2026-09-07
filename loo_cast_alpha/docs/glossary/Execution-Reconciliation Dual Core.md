---
canonical_name: Execution-Reconciliation Dual Core
status: WIP-draft
aliases: []
---

Execution-Reconciliation Dual Core means the runtime core has two irreducible functions: code execution and
intent reconciliation.
Execution produces candidate intents/outcomes; reconcile/commit/apply rules decide what becomes valid authoritative
runtime progression.
Neither side is reducible to the other.

Capability relationship:
[[Capability]] objects emit intents, relay requests, and expose structured authority.
The reconcile/commit/apply layer is outside the capability graph, but capability types may need to declare enough
metadata for the host executor to reconcile their intents correctly.
Leaf capabilities may bind directly to Rust functions/types, including mutable operations, but canonical state
progression is still decided by host-side reconciliation and/or general processing.

See also:

- [[USF Runtime Evolution Lifecycle]]
- [[Project Runtime Representation]]
- [[Capability Runtime]]

#glossary
