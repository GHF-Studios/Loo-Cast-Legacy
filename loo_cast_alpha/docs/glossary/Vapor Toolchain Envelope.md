---
canonical_name: Vapor Toolchain Envelope
status: WIP-draft
aliases:
  - Toolchain Envelope
  - Toolchain Lock
---

The Vapor Toolchain Envelope is the locked Rust/cargo/platform/toolchain compatibility envelope needed for Vapor
native kernel artifacts.

Rust dynamic linking is not automatically equivalent to ordinary Cargo dependency use.
Normal Rust dependencies are type-checked, monomorphized, and linked in one compile-time crate graph.
Runtime-loaded Rust dynamic libraries are compiled separately and discovered later.
Generics, blanket impls, trait objects, duplicated dependencies, layout assumptions, and compiler-private ABI details
therefore need a deliberate compatibility boundary.

Current direction:

- The SDK should control supported Rust/cargo/toolchain branches.
- Vapor may need to vendor canonical toolchains or act as a Rust meta-toolchain launcher for Vapor projects.
- [[Kernel Artifact]]s must be built inside a known compatible Vapor toolchain envelope.
- Loading should use a narrow Rust-native registration entrypoint, then immediately move into Vapor-owned Rust registry
  contracts.
- The toolchain envelope must make the Rust-native registration entrypoint compatible enough to use. If compatibility
  cannot be proven, loading fails fast.
- A C ABI entrypoint is not accepted for Vapor kernel loading; it would only prove a trivial FFI-safe import boundary,
  not the Rust-native cross-binary interaction Vapor needs.
- [[Vapor.lock]] should record the resolved toolchain, platform, ABI envelope, native dependency, and kernel fingerprint
  material needed to prove compatibility.

Boundary:
The toolchain envelope makes Rust-native dynamic loading plausible.
It does not remove the need for explicit plugin boundaries, registration metadata, and host contracts.

See also:

- [[Kernel Artifact]]
- [[Capability Kernel]]
- [[Vapor.lock]]
- [[Rust Surface Graph]]
- [Vapor Product-Stack Proof Overview](../diagrams/overview.puml)

#glossary
