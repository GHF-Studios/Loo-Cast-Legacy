---
canonical_name: Capability Declaration
status: WIP-draft
aliases: [ ]
---

The Capability Declaration is the pre-lock authored declaration payload for a [[Capability]].

It is data-first (POD-oriented) with declared behavior payload and metadata shaped by a target
[[Capability Slot Type]] / Rust host contract.
When callbacks are declared, callback access policy inputs must resolve into effective callback `ctx` path masks before
[[Runtime Lock]].
These runtime callback masks remain bounded by the [[Capability Graph Scope Envelope]].

One [[Rhai Asset]] file defines exactly one authored leaf capability declaration node.
File-internal capability definitions should be private/internal-only by default.
Folder-level Rhai aggregators may group declarations and carry metadata without making every file-internal construct a
public graph node.
At the definition lock transition, each validated capability declaration is promoted into a [[Capability]].
Canonical lifecycle, Rust/Rhai loop, and multiplicity semantics are defined in [[Capability]].

Slot-type boundary:
A Capability Declaration does not automatically define a new [[Capability Slot Type]].
It may define one only through explicit opt-in metadata/host support.

First-order declaration contexts are root-level and are forbidden from depending on other capabilities.

Workflows should orchestrate lifecycle around validated capabilities, artifact boundaries, and contract boundaries, not
raw script-engine internals.

#glossary
