---
canonical_name: Vapor Crate Topology
status: WIP-draft
aliases: []
---

Vapor Crate Topology is the current implementation-facing pressure for how the [[Vapor Ecosystem]] may be organized as
a Rust workspace/crate family.

Current owner-answer-informed direction:

- Vapor should be able to become public/open-source early, while first-party engine/game code can remain proprietary
  until a later decision.
- Vapor should likely be split into multiple crates rather than one monolithic crate.
- Candidate crate names include `vapor_core`, `vapor_sdk`, `vapor_launcher`, `vapor_steam`, and `vapor_macros`.
- `vapor_sdk` and `vapor_launcher` should be siblings over `vapor_core`, not one hidden inside the other.
- `vapor_core` should contain the capability graph implementation, Vapor.toml parser, fingerprinting, validation
  primitives, shared contracts/types, and shared utility surfaces for engines, games, mods, packagepacks, and related
  concepts.
- `vapor_core` should define traits and shared types that `vapor_steam` implements and integrates with.
- If Phase 3 uses macro-backed capability metadata, `vapor_macros` should be public API.
- `vapor_macros` dependency direction is not locked; if internal-only macros are needed later, they can live in a
  separate internal macro crate.

Phase 3 lock-candidate anchor:
The active execution anchor is [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially
P3-W01.
Required Phase 3 crate targets are `vapor_core`, `vapor_sdk`, `vapor_launcher`, and `vapor_steam` unless an explicit
owner-approved rename replaces them.
`vapor_macros` exists in Phase 3 only if macro-backed capability metadata is used.

Boundary:
Vapor is not best described as crate first or product/ecosystem first.
The crate/workspace shape, product layer, ecosystem layer, SDK, launcher, and distribution surface are all critical
parts of the same Vapor concept.

#glossary
