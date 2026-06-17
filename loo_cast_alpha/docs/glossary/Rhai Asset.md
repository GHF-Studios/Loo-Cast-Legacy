---
canonical_name: Rhai Asset
status: WIP-draft
aliases:
  - Rhai Declaration Asset
---

A Rhai Asset is a canonical authored asset represented as a [[Rhai]] declaration file.
Current owner-answer-informed doctrine is strong: a Rhai declaration file is always an asset, and an authored asset is
always a Rhai declaration.
Current capability semantics push this further: all assets are capabilities, and all capabilities are assets, at least
for the authored/declaration model.

File/node shape:

- One Rhai declaration file should represent one authored leaf asset/capability node.
- The file is a different container artifact type, not proof that the node is graph-top-level.
- File-internal capability definitions should be private/internal-only by default.
- Folder/module-style assets may use Rhai-level `mod.rs`-like aggregator files for grouping and metadata.
- Ordinary asset/capability metadata should live on the Rhai file/folder declaration surface, not in sidecar `.meta`
  files.

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
The rendering/audio/model generation stack is underexplored.
Generated media may still need rasterized or otherwise concrete delivery artifacts for GPU/audio/runtime execution.
The current direction implies procedural or physically/world-state-derived rendering, audio, and model generation, but
the concrete architecture is not yet settled.
The relationship between Rhai-side capability usage and Rust-side capability kernel usage needs a dedicated follow-up
pass before it becomes doctrine.

#glossary
