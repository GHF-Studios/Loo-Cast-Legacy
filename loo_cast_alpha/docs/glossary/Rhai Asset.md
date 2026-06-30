---
canonical_name: Rhai Asset
status: WIP-draft
aliases:
  - Rhai Declaration Asset
---

A Rhai Asset is a canonical authored [[Rhai]] source file that contributes typed Rhai-side source material to the
[[Capability Framework]].
It is now an umbrella over specialized Rhai source-file roles rather than proof that one file equals one capability.

Primary-source boundary:
Rhai declarations are foundational, but they are not the only primary source form.
Rust-defined hardcoded or type-system-bound capabilities and Rhai-defined data-oriented capabilities are both primary
inputs into the dynamic heterogeneous [[Capability]] graph.

File/node shape:

- One Rhai file is one typed declaration asset, not necessarily one [[Capability Node]] or one
  [[Capability Instance]].
- [[Vapor.toml]] classifies each Rhai file's role, for example capability type, capability trait, or capability
  callback material.
- File and folder names should remain freely choosable where practical; manifest classification is the source of truth.
- Reserved typed folders such as `types`, `traits`, `callbacks`, and `capabilities` make the module layout readable
  without making arbitrary folder structure semantic by default.
- [[Vapor.toml]] is now allowed and expected for manifest-style metadata such as dependencies, conflicts, target roles,
  version requirements, and Steam/Workshop publication fields.
- Separate sidecar `.meta` files remain disfavored; Vapor.toml is the explicit manifest surface when manifest data is
  needed.

Current specialized Rhai asset roles:

- Rhai Capability Type Asset: defines [[Capability Type]] material.
- Rhai Capability Trait Asset: defines [[Capability Trait]] material.
- Rhai Capability Callback Asset: defines [[Capability Callback]] material.
- Rhai Capability Node/Module Asset: possible umbrella wording for files that contribute to a [[Capability Node]] or
  [[Capability Module]], but the module itself is folder/manifest structure rather than one Rhai file.
- Rhai Data/Config/Localization Asset: typed data payloads such as configuration, localization, constants, tuning
  values, authored tables, and other capability-framework-relevant data that is not native implementation code.

Boundary:
Rhai is for declaring capability-relevant source material and framework-relevant data.
Data assets are ordinary structured payloads: configuration, localization, constants, tuning values, authored tables,
and similar non-native inputs.
Capability declarations are broader than data assets: they may carry data, metadata, links, graph meaning, and callback
declarations.
The only declaration-level logic that should cross the capability boundary is sanctioned [[Capability Callback]]
material.
Callbacks are host-invoked and policy mediated.
[[Vapor.toml]] is the Cargo.toml-like build/package metadata equivalent for folder/artifact structure, dependencies,
attachment, and publication metadata.
Rhai authoring contexts should be generated from Vapor.toml, data-role metadata, type/trait/callback metadata, and
capability graph policy.

Rust/native implementation pressure:
Native implementation backing belongs in [[Capability Kernel]]s and [[Scriptable Rust Surface]]s, not in Rhai assets.
One Rhai file is a typed declaration or data asset; one kernel artifact may back many declared types, traits,
callbacks, and nodes.

Traditional media payloads such as textures, models, and sounds should not be treated as canonical authored assets in the
normal model.
They are generated, cached, rendered, synthesized, or packaged outputs derived from declarations and runtime/world state.

Motivation:
Scale-continuous traversal makes fixed traditional textures, models, and audio assets structurally awkward.
Procedural and physically/world-state-derived representation is expected to preserve continuity across zoom, scale, and
detail materialization better than fixed media payloads.

Open pressure:
The relationship between Rhai-side capability usage and Rust-side [[Capability Kernel]] usage needs continued audit.
Platform-level Rhai callbacks for authoring, validation, publishing, or launcher lifecycle hooks are in scope; runtime
gameplay-style Rhai callbacks remain out of scope for the current Vapor-focused pass.
Rhai hooks are expected to exist in Phase 3.
[[Capability Callback]]s are scheduled/mediated entrypoints with firing policy, such as single-fire, multi-fire, or
procedural-fire behavior.
Generic capability functions are not active doctrine.

Phase 3 lock-candidate anchor:
Rhai Asset Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W04.
Phase 3 must load and validate Rhai declarations without launching a concrete Engine/Game fixture, map typed Rhai
declaration/data payloads into capability/fingerprint paths, and prove one focused callback path without locking the
full callback taxonomy.

Diagram:
[Vapor Product-Stack Proof Overview](../diagrams/overview.puml)

#glossary
