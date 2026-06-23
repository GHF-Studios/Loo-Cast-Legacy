---
canonical_name: SDK
status: WIP-draft
aliases: []
---

The SDK is the full creator-facing toolchain surface of the [[Vapor Ecosystem]], not merely a code library.
It supports authoring, validating, linting, packaging, fingerprinting, building, publishing, updating, migrating, and
documenting engines, games, mods, modpacks, packagepacks, and related Vapor artifacts.

Current owner-answer-informed direction:

- The SDK should be accessible through launcher UI, CLI commands, and Rust workspace tooling.
- SDK CLI commands should exist before the launcher UI, while the full launcher remains part of Phase 3 acceptance.
- `xtask` remains internal repo/root-level first-party development tooling; Vapor SDK is the public creator tooling.
- The SDK and launcher should be siblings over a shared `vapor_core`, not one built as a thin layer inside the other.
- SDK scaffolding should generate Engine, Game, Mod, Modpack, and [[Packagepack]] project skeletons.
- SDK scaffolding should enforce reserved role names and folder layout where Vapor requires them.
- SDK validation/lint commands should work without launching a concrete engine/game fixture.
- SDK package/fingerprint/build commands are needed for local authoring.
- SDK publish/update commands are needed for [[Steam Workshop]].
- SDK migration tooling is needed for Vapor metadata/schema changes.
- LSP/editor support is desirable where the public authoring surface benefits from it, but exact Phase 3 scope is not
  locked.
- A local content registry should begin as discovered workspace folders with a simple working index.
- Documentation generation from [[Vapor.toml]], [[Rhai Asset]], and [[Capability]] metadata is desirable, starting
  barebones but conceptually similar to `cargo doc`; this is currently Phase 3.5 quality-layer pressure rather than a
  Phase 3 hard requirement.
- SDK command semantics should be stable public contracts by Phase 3.
- SDK command surface/matrix means the concrete set of public CLI/tool commands and examples, not a mathematical matrix.
- The SDK may need to manage the [[Vapor Toolchain Envelope]] for native [[Kernel Artifact]] builds, including supported
  Rust/cargo/toolchain branches.
- Toolchain control may mean vendoring one or more canonical Rust/cargo branches, or acting as a Rust meta-toolchain
  launcher for Vapor projects.

Boundary:
The SDK should share semantics with [[Vapor Launcher]] operations through the same [[Capability]]/[[Rhai]]/[[Vapor.toml]]
validation model where practical.
Phase 3 should still produce a small but real SDK command surface with examples.
CLI commands should exist before launcher UI, but launcher acceptance remains required.

Phase 3 lock-candidate anchor:
The SDK scope for Phase 3 is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W05.
Minimum Phase 3 command families are scaffold, validate, fingerprint/dump, build/package, publish/update, install,
update, enable, disable, uninstall, and local metadata/schema migration where useful for authoring.
These commands are public creator-facing contracts by Phase 3, even if implementation internals remain rough.

See also:

- [[Vapor Crate Topology]]
- [[Vapor Launcher]]
- [[Vapor Toolchain Envelope]]
- [[Kernel Artifact]]

#glossary
