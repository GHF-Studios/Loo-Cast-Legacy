---
canonical_name: Rhai Asset
status: WIP-draft
aliases:
  - Rhai Declaration Asset
---

A Rhai Asset is a canonical authored asset/capability declaration surface represented as a [[Rhai]] declaration file.
Current owner-answer-informed doctrine is strong for authored declaration files: one Rhai declaration file should map to
one authored leaf capability declaration by default.
Batch 005 correction:
Rhai declarations are foundational, but they are not the only primary source form.
Rust-defined hardcoded or type-system-bound capabilities and Rhai-defined data-oriented capabilities are both primary
inputs into the dynamic heterogeneous [[Capability]] graph.

File/node shape:

- One Rhai declaration file should represent one authored leaf capability declaration node.
- The file is a different container artifact type, not proof that the node is graph-top-level.
- File-internal capability definitions should be private/internal-only by default.
- Folder/module-style assets may use Rhai-level `mod.rs`-like aggregator files for grouping capability declarations and
  module-like structure.
- [[Vapor.toml]] is now allowed and expected for manifest-style metadata such as dependencies, conflicts, target roles,
  version requirements, and Steam/Workshop publication fields.
- Separate sidecar `.meta` files remain disfavored; Vapor.toml is the explicit manifest surface when manifest data is
  needed.

Boundary:
Rhai is for declaring capabilities.
[[Vapor.toml]] is the Cargo.toml-like build/package metadata equivalent for folder/artifact structure, dependencies,
attachment, and publication metadata.
Rhai authoring contexts should be generated per capability kind from Vapor.toml and capability metadata.

Rust kernel topology pressure:
Rust leaf capability kernels should also tend toward one file per leaf kernel.
`Leaf`, `Atomic`, and `BareMetal` are currently near-synonyms in this topology pressure.

Traditional media payloads such as textures, models, and sounds should not be treated as canonical authored assets in the
normal model.
They are generated, cached, rendered, synthesized, or packaged outputs derived from declarations and runtime/world state.

Motivation:
Scale-continuous traversal makes fixed traditional textures, models, and audio assets structurally awkward.
Procedural and physically/world-state-derived representation is expected to preserve continuity across zoom, scale, and
detail materialization better than fixed media payloads.

Open pressure:
The relationship between Rhai-side capability usage and Rust-side capability kernel usage needs follow-up discussion.
Platform-level Rhai callbacks for authoring, validation, publishing, or launcher lifecycle hooks are in scope; runtime
gameplay-style Rhai callbacks remain out of scope for the current Vapor-focused pass.
Rhai hooks are expected to exist in Phase 3.
Callbacks are currently best understood as ways to define logic entrypoints with firing policy, such as single-fire,
multi-fire, or procedural-fire behavior, but exact hook taxonomy remains open.

Phase 3 lock-candidate anchor:
Rhai Asset Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W04.
Phase 3 must load and validate Rhai declarations without launching a concrete Engine/Game fixture, map Rhai declaration
data into capability/fingerprint paths, and prove one focused callback path without locking the full callback taxonomy.

#glossary
