---
canonical_name: Kernel Artifact
status: WIP-draft
aliases:
  - Native Kernel Artifact
---

A Kernel Artifact is a packaged native implementation artifact that provides one or more [[Capability Kernel]]s.
For Phase 3 pressure, the main candidate form is a platform-specific dynamic library or shared object associated with a
[[Capability Module]].

Plugin-boundary direction:
Use the narrowest possible loading boundary.
The required shape is a small Rust-native entrypoint that registers Rust-native surfaces into `vapor_core`, after which
normal Vapor-owned Rust registry contracts own meaningful interaction.
This entrypoint must be proven compatible by the locked [[Vapor Toolchain Envelope]] and [[Vapor.lock]] metadata.
If that Rust-native registration boundary cannot be proven compatible, the kernel artifact is invalid and loading must
fail fast.
Do not accept a C ABI entrypoint for Vapor kernel loading; a C-only boundary would not prove the Rust-native
cross-binary interaction model Vapor actually needs.

Disallowed for kernel loading:

- ad hoc cross-library FFI calls for ordinary execution
- C-safe vtables as the normal internal call model
- C ABI entrypoints used instead of proven Rust-native compatibility
- treating toolchain lock as permission to skip registration metadata, host contracts, or compatibility proof
- exposing raw private kernel internals to dependent modules
- assuming arbitrary Rust dylibs can be used like ordinary Cargo dependencies without a toolchain/ABI envelope

Required compatibility pressure:
[[Vapor.lock]] should eventually record kernel artifact identity, platform target, ABI/toolchain envelope, dependency
fingerprints, exported registration metadata, and compatibility proof.

See also:

- [[Capability Kernel]]
- [[Vapor Toolchain Envelope]]
- [[Vapor.lock]]
- [[Rust Surface Graph]]
- [[Rust Host Contract]]
- [Vapor Product-Stack Proof Overview](../diagrams/overview.puml)

#glossary
