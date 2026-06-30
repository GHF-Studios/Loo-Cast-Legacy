---
canonical_name: Capability Bootstrap Fixed-Point Cycle
status: WIP-draft
aliases: []
---

The Capability Bootstrap Fixed-Point Cycle is the iterative topological startup process that progressively
materializes resolvable capability/API layers.
Layers are dynamic from concrete topology, but each pass is strict and deterministic.
First-order capabilities are declaration-level roots and are forbidden from depending on other capabilities.
Dependency cycles are invalid.

See also:

- [[Global Capability API Graph]]
- [[Capability Resolution Semantics]]
- [[USF Definition Lifecycle]]
- [[Runtime Lock]]

Implementation-facing anchor:
[Vapor Product-Stack Proof Overview](../diagrams/overview.puml)

#glossary
