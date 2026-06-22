---
canonical_name: Capability Declaration
status: WIP-draft
aliases: [ ]
---

The Capability Declaration is the authored pre-materialization payload for a [[Capability]] / [[Capability Instance]].

It is data-first (POD-oriented) with declared behavior payload and metadata shaped by a target
[[Capability Slot Type]] / Rust host contract.
When callbacks are declared, callback access policy inputs must resolve into effective callback `ctx` path masks before
[[Runtime Lock]].
These runtime callback masks remain bounded by the [[Capability Graph Scope Envelope]].

One [[Rhai Asset]] file defines exactly one authored leaf capability declaration node.
File-internal capability definitions should be private/internal-only by default.
Folder-level Rhai aggregators may group declarations and carry metadata without making every file-internal construct a
public graph node.
During iterative/topological startup, validated declarations may be promoted into staged [[Capability Instance]]s before
final [[Runtime Lock]].
Canonical lifecycle, Rust/Rhai loop, and multiplicity semantics are defined in [[Capability]].

Slot-type boundary:
A Capability Declaration does not automatically define a new [[Capability Slot Type]].
It may define or implement one only through explicit opt-in metadata and host support.
This is one reason Rhai-authored assets may later need a clearer split between capability declarations, callback
declarations, slot-type declarations, and other declaration asset classes.

First-order declaration contexts are root-level and are forbidden from depending on other capabilities.

Workflows should orchestrate lifecycle around validated capabilities, artifact boundaries, and contract boundaries, not
raw script-engine internals.

#glossary
