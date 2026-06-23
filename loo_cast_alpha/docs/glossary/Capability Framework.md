---
canonical_name: Capability Framework
status: WIP-draft
aliases:
  - Scripting Framework
---

The Capability Framework is the Vapor-owned typed graph, declaration, callback, trait, extension-slot, projection, and
runtime binding framework.
`Scripting Framework` is an informal alias only.
It is useful because Rhai participates in data authoring, declaration authoring, wiring, and callback authoring, but it
must not imply that Rhai owns heavy execution, native integration, lifecycle scheduling, or canonical state authority.

The framework spans:

- [[Capability Module]] and [[Capability Node]] source organization
- [[Capability Type]], [[Capability Trait]], [[Capability Callback]], and [[Capability Extension Slot]] declarations
- signature metadata such as [[Capability Type Signature]], [[Capability Trait Signature]],
  [[Capability Instance Signature]], and [[Callback Signature]]
- [[Rhai Asset]] data assets such as config/localization and declaration assets whose logic surface is callbacks
- [[Rust Host Contract]] validation, projection, materialization, and binding rules
- [[Scriptable Rust Surface]] registration into the [[Rust Surface Graph]]
- [[Capability Kernel]] and [[Kernel Artifact]] integration where native implementation backing is needed
- post-lock runtime behavior through [[Capability Runtime]] and [[Runtime Substrate]]

Boundary:
The Capability Framework is not a general-purpose "functions framework."
It models capability-relevant abstractions, data assets, declaration payloads, callbacks, native bindings, visibility,
compatibility, and graph authority.
Declarations may contain data and metadata, but executable declaration behavior crosses the framework boundary only as
sanctioned [[Capability Callback]]s.
Generic utility functions remain ordinary code unless they are explicitly exposed as Scriptable Rust Surfaces or
capability declarations.

See also:

- [[Capability]]
- [[Rhai Asset]]
- [[Rust Host Contract]]
- [[Scriptable Rust Surface]]
- [[Capability Kernel]]
- [[Capability Runtime]]

#glossary
