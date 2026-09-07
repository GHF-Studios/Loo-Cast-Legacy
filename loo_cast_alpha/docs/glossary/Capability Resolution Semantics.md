---
canonical_name: Capability Resolution Semantics
status: WIP-draft
aliases: []
---

Capability Resolution Semantics defines how capabilities/APIs are resolved from declaration/bootstrap into locked
runtime state without collapsing dependency resolution and projection/access resolution into one layer.

Resolution is intentionally multi-layered:

1. Dependency-graph resolution (bootstrap):
   dynamic strict layered passes resolve currently satisfiable nodes from concrete topology.
   First-order declarations are root-level and cannot depend on other capabilities.
   Cycles are invalid.
   Dependency resolution strategy is policy-ordered:
   host type-system enforcement first, selective explicit dependency declarations second, and selective inference
   fallback third.
2. Materialization/merge resolution:
   validated declaration outputs are materialized and merged into the host graph and the process repeats until
   fixed-point convergence.
3. Projection/access resolution:
   declaration/callback-scoped projection APIs and allow/deny path policy determine accessible contextual facades.
   Declaration and callback projection contexts are co-equal projections, not implicit parent/child scope layers.
   This layer may narrow/deny/re-open paths, but cannot widen authority beyond
   [[Capability Graph Scope Envelope]].

Hard-fail conditions include unresolved required dependencies, cycle detection, invalid multiplicity, or denied required
callback paths after policy resolution.
No implicit fallback dependency owner is introduced.

See also:

- [[Capability Bootstrap Fixed-Point Cycle]]
- [[Capability Path]]
- [[Capability Projection API]]
- [[Capability Graph Scope Envelope]]

#glossary
