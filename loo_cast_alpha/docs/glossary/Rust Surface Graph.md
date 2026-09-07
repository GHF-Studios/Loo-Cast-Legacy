---
canonical_name: Rust Surface Graph
status: WIP-draft
aliases:
  - Native Capability Registry
  - Rust Metadata Graph
---

The Rust Surface Graph is the Rust-native registry/graph of scriptable host functionality known to Vapor.
It is expected to live in or be coordinated by `vapor_core`, with launched compositions such as `core_engine`
instantiating or initializing it rather than inventing their own capability framework.

It records selected native surfaces from built-in Vapor code, first-party engines, custom crates, external crates, and
loaded [[Kernel Artifact]]s.
Those surfaces become [[Scriptable Rust Surface]] entries that can be projected into the capability/Rhai side through
[[Rust Host Contract]]s.

Graph layers:

- outer graph: folder/module/package hierarchy and [[Capability Module]] source layout
- inner graph: kernel artifact exported metadata or registration tables
- inner-inner graph: mutable runtime substrate/registry/state graph allowed after [[Runtime Lock]]

Boundary:
The Rust Surface Graph is host-authoritative.
The Rhai/capability declaration side sees projected, typed, policy-governed views of it, not raw global native access.

See also:

- [[Capability Framework]]
- [[Scriptable Rust Surface]]
- [[Rust Host Contract]]
- [[Capability Runtime]]
- [[Runtime Substrate]]

#glossary
