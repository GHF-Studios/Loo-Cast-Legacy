---
canonical_name: Capability Declaration
status: WIP-draft
aliases: [ ]
---

The Capability Declaration is the singleton-like script-produced declaration payload for one [[Capability Profile]]
identity.

It is data-first (POD-oriented) with declared behavior payload and metadata shaped by a target
[[Capability Type Template]].
When callbacks are declared, callback access policy inputs must resolve into effective callback `ctx` path masks before
[[Runtime Lock]].
These runtime callback masks remain bounded by the [[Capability Graph Scope Envelope]].

One [[Rhai Asset]] file defines exactly one authored capability declaration node.
That declaration may contain sub-declarations or intra-file structure, but the file container should still represent one
node like any other.
At the definition lock transition, each validated capability declaration is promoted into a [[Capability]].
Canonical lifecycle, Rust/Rhai loop, and multiplicity semantics are defined in [[Capability]].

First-order declaration profiles are root-level and are forbidden from depending on other capabilities.

Workflows should orchestrate lifecycle around materialized runtime artifacts and contract boundaries, not raw
script-engine internals.

#glossary
