# Contracts

Purpose:

- Define what external code/data may rely on.
- Define when a change is breaking.
- Define required migration behavior.

Current pre-alpha boundary caveat:

- The active product stack is `Vapor Ecosystem -> Spacetime Engine -> Loo Cast`.
- Vapor is Steam-exclusive and includes SDK/tooling plus runtime protocol/contracts/traits for engines, games, mods, and
  packages.
- USF is a public/API-facing Spacetime Engine subsystem, not a standalone product contract pillar.
- `core_engine`, matching `core_mod`, and `base_mod` are reserved built-in mod roles and mandatory pillars of a Vapor
  product instance stack.
- `core_engine` plus matching `core_mod` define a coupled mandatory Engine pillar.
- `core_mod` is mandatory but replaceable by selecting another valid coupled Engine pair.
- `base_mod` is mandatory but replaceable as the Game layer for Loo Cast.
- This file is a contract planning surface; the latest glossary and owner-confirmed answers outrank stale wording here.

Glossary:

- Published game version
  - `vX.Y.Z` (stable) or `vX.Y.Z-rc.N`/`vX.Y.Z-beta.N` (pre-release) tag on `main`.
  - CI-built immutable artifacts from that exact tag.
  - Artifacts pushed to a named distribution channel.
  - Not the same as an arbitrary `develop` commit.

- Contract version
  - Version of the frozen contract set.
  - Locked to published game version (lockstep versioning).

- Breaking change
  - Any change that can invalidate previously compatible mods, saves, manifests, or load behavior.

- Compatible mod
  - Mod that declares support for the target contract version and passes build/load checks.

- Migration guide
  - Formal, versioned document for a breaking change once stable-contract mode is active.
  - Must provide upgrade steps or explicit "no migration path".

Frozen contract set (per published game version):

- Public `mod_api` surface.
- Mod manifest schema.
- Mod package/load-order rules.
- Save-data schema + compatibility semantics.

Publish policy:

- Stable publish requires all of:
  - `main` tag `vX.Y.Z`
  - immutable artifacts built from tag
  - release pushed to stable channel

- Pre-release publish requires all of:
  - `main` tag `vX.Y.Z-rc.N` or `vX.Y.Z-beta.N`
  - immutable artifacts built from tag
  - release pushed to non-stable channel

- Immutability rules:
  - no retagging published versions
  - no replacing published artifacts in-place
  - supersede by publishing a new version only

Version policy:

- Use `MAJOR.MINOR.PATCH` for published game/contract versions.
- `0.y.z` is initial development. Public API stability is not guaranteed.
- `0.5.0` is the alpha milestone: the feature framework, mod/API shape, and execution rails are coherent enough for
  sustained feature and gameplay development.
- `0.9.0` is the beta milestone: feature/framework direction is mostly locked, and work shifts toward bug fixing,
  refinement, compatibility, packaging, and release hardening.
- `1.0.0` is the stable commercial release baseline: the public API is defined, compatibility rules become stricter, and
  the project can move toward its finalized open-source/licensing structure.
- After `1.0.0`, `PATCH`: backward-compatible fixes only.
- After `1.0.0`, `MINOR`: backward-compatible additions only.
- After `1.0.0`, `MAJOR`: breaking changes allowed.
- No contract break is allowed inside one published version.
- Any contract break requires a new published version.

Lockstep version topology:

- Shared version tag: `core_engine`, `mod_api` (when present), `core_mod`, `base_mod`.
- Third-party mods version independently.
- Third-party mods must declare compatible contract range.

Migration policy:

- Current structural phase (active now):
  - Breaking-shape changes are expected while architecture/contracts are still settling.
  - Track migration-impact notes in `MIGRATIONS_DRAFT.md`.
  - Track release-note intent in `CHANGELOG_DRAFT.md`.
- Stable-contract mode (activated by explicit decision):
  - Breaking PRs must target a new published version.
  - After `1.0.0`, breaking PRs must target a new `MAJOR` (or pre-release of that `MAJOR`).
  - Breaking PRs must include a migration guide at `docs/migrations/<from>-to-<to>.md`.
  - Breaking PRs must update `CONTRACTS.md` and `DECISIONS.md`.
  - Guide must include:
    - affected contracts
    - who is impacted
    - exact upgrade steps
    - fallback/rollback notes
