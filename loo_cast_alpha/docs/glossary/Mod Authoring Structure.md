---
canonical_name: Mod Authoring Structure
status: WIP-draft
aliases: []
---

The Mod Authoring Structure defines the source-side folder and file layout of a mod.
Authoring includes two required Rust source parts: dynamic-library implementation source and contract-crate source, even
when either side is effectively no-op.
Authoring may additionally include declaration bundles, with Rhai declaration sources as the canonical authored
data/asset-definition layer.
Materialized media payloads (for example textures or audio buffers) are runtime-generated or runtime-cached outputs
rather than canonical authored source.
Typed script declarations are authored against contextual APIs rather than unrestricted raw runtime access, and are
aligned with the [[SDK]], the [[Modding Contract]], and the [[Capability Contract]].

#glossary
