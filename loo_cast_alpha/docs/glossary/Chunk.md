---
canonical_name: Chunk
status: WIP-draft
aliases: []
source_of_truth: []
---

A Chunk is the first-level USF spatial partition at a canonical [[Scale]].
Chunks are scale-native and nested exactly by scale; each canonical scale uses chunks with `1000^3` scale-local units as
the current hard USF law.

Chunks are not generic engine chunks.
They are core USF structure and may act simultaneously as spatial address, cache boundary, generation unit, simulation
unit, and realization unit.
The relationships among those roles are intentionally not collapsed into one simple authority statement.

Current owner-answer-informed model:
[[Phenomenon]]-level structures are the primary carriers of significant world generation and state.
Chunks may carry highly insignificant, highly distributed, or local residual state before it crosses a
phenomenon-defined significance threshold.
Concrete significant state changes should be persisted or represented through phenomena rather than treating chunk
storage as the primary world authority.

Internal partitioning:
Within a chunk, real runtime structures may use BSPs, octrees, sparse fields, grids, cellular automata, or other
adaptive/per-content layouts.
Those internal structures may evolve dynamically to fit chunk contents, but they remain second-level internal
partitioning beneath the scale/chunk hierarchy.

Open pressure:
`chunk authority` is ambiguous.
It may be better to speak about local operational authority, residual state, generation/cache authority, or
phenomenon-thresholded canonicalization depending on the specific context.

#glossary
