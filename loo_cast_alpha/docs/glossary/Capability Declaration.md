---
canonical_name: Capability Declaration
status: WIP-draft
aliases: [ ]
---

The Capability Declaration is the raw authored form of a [[Capability Node]] before validation/materialization into a
[[Capability Instance]].

It is no longer modeled as "one Rhai file equals one capability declaration."
The declaration may be spread across a [[Capability Module]] / Capability Node source subtree with typed files for
[[Capability Type]]s, [[Capability Trait]]s, and [[Capability Callback]]s plus manifest metadata in [[Vapor.toml]].

Declared material is data-first (POD-oriented): data, metadata, links, graph meaning, and callback declarations shaped
by explicit type, trait, callback, extension-slot, and Rust host contracts.
Declaration-level behavior crosses the capability boundary only as sanctioned [[Capability Callback]] material.
When callbacks are declared, callback access policy inputs must resolve into effective callback `ctx` path masks before
[[Runtime Lock]].
These runtime callback masks remain bounded by the [[Capability Graph Scope Envelope]].

During iterative/topological startup, validated declarations may be promoted into staged [[Capability Instance]]s before
final [[Runtime Lock]].
Canonical lifecycle, Rust/Rhai loop, and multiplicity semantics are defined in [[Capability]].

Source-shape boundary:
A Capability Declaration can define or reference Capability Types, Capability Traits, Capability Callbacks, child
Capability Nodes, and [[Capability Extension Slot]] relationships only through explicit source/manifest metadata and
host support.
Plain file placement or arbitrary folder nesting does not silently create type, trait, callback, inheritance, or
execution semantics.

First-order declaration contexts are root-level and are forbidden from depending on other capabilities.

Workflows should orchestrate lifecycle around validated capabilities, artifact boundaries, and contract boundaries, not
raw script-engine internals.

#glossary
