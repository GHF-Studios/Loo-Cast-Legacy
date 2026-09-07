---
canonical_name: Chunk
status: WIP-draft
aliases: []
---

A Chunk is the first-level [[USF]] spatial partition at a canonical [[Scale]].
Chunks are scale-native and nested exactly by scale; each canonical scale uses chunks with `1000^3` scale-local units as
the current hard USF law.
Chunk size does not vary per scale.

Chunks are not generic engine chunks.
They are core USF structure and may act simultaneously as spatial address, cache boundary, generation unit, simulation
unit, and realization unit.
The relationships among those roles are intentionally not collapsed into one simple authority statement.
Chunks should be invisible to normal content authors most of the time.

Current owner-answer-informed model:
[[Phenomenon]]-level structures are the primary carriers of significant world generation and state.
Chunks may carry highly insignificant, highly distributed, local residual, cache, temporary simulation, and
metric-oriented state before it crosses a phenomenon-defined significance threshold.
Concrete significant state changes should be persisted or represented through phenomena rather than treating chunk
storage as the primary world authority.
Chunk state may still be persisted for cache/performance purposes without becoming canonical world authority.
Normal content authors should not need to think about chunks most of the time.
Some distributed states, such as gravity-like metric fields, may be true runtime state that naturally lives through the
chunk/metric substrate rather than just as ordinary localized phenomenon state.

Internal partitioning:
Within a chunk, real runtime structures may use BSPs, octrees, sparse fields, grids, cellular automata, or other
adaptive/per-content layouts.
Those internal structures may evolve dynamically to fit chunk contents, but they remain second-level internal
partitioning beneath the scale/chunk hierarchy.
Live representation changes are allowed but expensive.
The current preferred direction is that chunks expose the ability to switch representation, while a broader orchestrator
decides when such switches are globally justified across chunk context.

Open pressure:
`chunk local operational authority` is only a partial term.
Chunk cache deletion/regeneration invariants are still unclear because chunks may hold more than disposable caches.

#glossary
