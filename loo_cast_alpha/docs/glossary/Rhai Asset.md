---
canonical_name: Rhai Asset
status: WIP-draft
aliases:
  - Rhai Declaration Asset
---

A Rhai Asset is a canonical authored asset represented as a [[Rhai]] declaration file.
Current owner-answer-informed doctrine is strong: a Rhai declaration file is always an asset, and an authored asset is
always a Rhai declaration.

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

#glossary
