DO NOT TRUST ANYTHING BEYOND THIS POINT. Legcay stuff.




# Loo-Cast

Loo-Cast is currently developed in the `loo_cast_alpha/` subtree. That subtree contains the active Rust workspace,
current workflow policy, and authoritative project docs for ongoing alpha work.

`loo_cast_legacy/` is a read-only archive of older code. It is useful for historical reference but is not authoritative
for current implementation decisions.

## Repository Map

- `loo_cast_alpha/`: active alpha workspace and delivery surface.
- `loo_cast_alpha/docs/`: canonical docs entrypoint for current direction, contracts, workflows, and decisions.
- `loo_cast_alpha/third_party/`: intentionally vendored third-party utilities used by xtask support commands.
- `loo_cast_legacy/`: legacy archive/museum; no active development target.
  - outer `loo_cast_legacy/` Rust/Bevy-era material: pre-issue/PR structured workflow period.
  - inner legacy archives (notably `loo_cast_legacy/LEGACY/` and nested legacy-history folders): Unity-era material.

## Quick Start (Active Alpha Workspace)

```bash
cargo xtask setup_sdk
cargo xtask build
cargo xtask run
cargo xtask audit
```

`cargo xtask ...` works from repository root via root-level alias shim and also works from inside `loo_cast_alpha/`.

## Docs Entrypoints

- Start with `loo_cast_alpha/docs/README.md` for read order.
- `loo_cast_alpha/docs/NOW.md` for current direction and scope.
- `loo_cast_alpha/docs/WORKFLOWS.md` for branch/PR/validation process rules.
- `loo_cast_alpha/docs/CONTRACTS.md` for compatibility and version policy.
- `loo_cast_alpha/docs/DECISIONS.md` for durable policy/architecture decisions.
- `loo_cast_alpha/docs/CHANGELOG_DRAFT.md` for pre-stable release-note drafting.
- `loo_cast_alpha/docs/MIGRATIONS_DRAFT.md` for pre-stable migration-impact drafting.
- `loo_cast_alpha/docs/RUSTDOC_BASELINE.md` for active-crate docs posture and deferred documentation boundaries.
