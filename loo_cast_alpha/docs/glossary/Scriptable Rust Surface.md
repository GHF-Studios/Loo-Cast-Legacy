---
canonical_name: Scriptable Rust Surface
status: WIP-draft
aliases:
  - Scriptable Surface
---

A Scriptable Rust Surface is selected Rust-native functionality made visible to the [[Capability Framework]] through
explicit registration metadata and host contracts.
It is the current pressure term for the "Scriptable" trait/adaptor family.

The surface may wrap:

- Rust functions
- Rust types and constructors
- Rust traits or trait-like capabilities
- Bevy systems or system-facing adapters
- Tokio/std/Steam/OS/custom-crate functionality
- callback registration/invocation surfaces

Two implementation modes are expected:

- owned Rust types can implement Vapor's Scriptable trait family directly
- foreign types or external crates are exposed through wrapper/adaptor/descriptor code because Rust orphan rules and
  external crate design prevent arbitrary direct impls

Boundary:
Only selected Rust functionality should become scriptable.
Exposing a Scriptable Rust Surface means Vapor can describe, validate, project, fingerprint, and diagnose that surface.
It does not mean Rhai gets raw access to the Rust implementation.

See also:

- [[Rust Surface Graph]]
- [[Rust Host Contract]]
- [[Capability Kernel]]
- [[Rhai Capability]]
- [[Capability Projection API]]

#glossary
