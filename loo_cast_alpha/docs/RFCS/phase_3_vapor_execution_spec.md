# Phase 3 Vapor Execution Spec

Status: Phase 3 lock-candidate planning draft.

Authority caveat:
This file is the current execution target for Phase 3 planning.
It is still subordinate to later owner answers and glossary changes, but it should supersede older Phase 3 drafts that
describe Phase 3 as USF implementation work.

Lock-candidate rule:
For Phase 3 scope, work packages, and acceptance, this file is the active anchor.
Glossary pages define terms; this file decides which term behavior must be proven in Phase 3.
If a glossary page and this spec conflict, pause and reconcile the conflict before Phase 3 coding.

Phase name:
Phase 3: Vapor Launcher and Product-Stack Proof.

Objective:
Implement a real Vapor platform slice that can discover, validate, compose, publish, install, and launch Packagepacks
through the SDK, launcher, capability/Rhai substrate, and Steam Workshop distribution rails.
Phase 3 completion requires public-oriented Vapor seams that are authorable, publishable, installable, launchable, and
diagnostically inspectable, not only log/fingerprint output.

Phase 3 must prove the Vapor product stack without implementing USF/worldmodel/gameplay systems.

## Hard Boundaries

In scope:

- Vapor crate/workspace foundation.
- SDK command surface.
- Launcher surface.
- Steam identity, ownership, and Workshop flows.
- Vapor.toml and Vapor.lock handling.
- Capability graph construction, staging, validation, visibility, projection, and Runtime Lock for launchable
  Engine/Game compositions.
- Rhai declaration loading, validation, and focused callback proof.
- Packagepack, Enginepack, Gamepack, Modpack, Engine Mod, Game Mod, and Extension Mod composition.
- Fingerprints, diagnostics, local content index, install ledger, and the Phase 3 Vapor Testing Suite.
- Small but real executable Engine/Game fixtures that exist only to prove product-stack mechanics.

Out of scope:

- USF implementation.
- Chunks, metrics, phenomena, zones, scales, worldmodel/entity APIs, save/load, rendering, audio, generated media, and
  traditional gameplay.
- Storefront abstraction beyond Steam.
- GitHub-backed registry.
- Phase 3.5 quality-layer work unless explicitly pulled into Phase 3 by owner decision.

Rule:
If a task needs a worldmodel concept to feel meaningful, it is not Phase 3 work.
If a task can be proven with logs, strings, fingerprints, files, diagnostics, and package/Workshop movement, it may be
Phase 3 work.

## Entry Criteria

- Phase 2 gate decision explicitly unlocks Phase 3 coding.
- This spec and the Phase 4 execution spec exist and have no known blocker contradictions.
- Current glossary pages for Vapor, Capability, Rhai, Packagepack/Enginepack/Gamepack/Modpack, Steam, Vapor.toml,
  Vapor.lock, Fingerprint, SDK, Launcher, and Runtime Lock are internally consistent enough to implement from.
- Stale Phase 3 USF-implementation issue/milestone drafts are replaced.
- The owner accepts that Phase 3 is a large platform phase, not a quick prototype.

## Target Architecture

Candidate crates:

- `vapor_core`: capability graph, Vapor.toml/Vapor.lock, fingerprints, validation primitives, pack data model, shared
  traits/types, diagnostics core, local index primitives.
- `vapor_macros`: public macro API for reflected/declared capability metadata if Phase 3 uses macro-backed metadata.
- `vapor_sdk`: public CLI/tooling surface over `vapor_core`.
- `vapor_launcher`: UI/process surface over `vapor_core`, sibling to `vapor_sdk`.
- `vapor_steam`: Steamworks/Workshop adapter implementing traits/types defined by `vapor_core`.

Current dependency pressure:

```text
vapor_steam  -> vapor_core
vapor_sdk    -> vapor_core + vapor_steam
vapor_launcher -> vapor_core + vapor_steam
Engine/Game fixtures -> vapor_core
```

`vapor_macros` dependency direction is not locked.
If it exists, it should be a public API crate for public macros, not a hidden internal macro surface.
Internal-only macros can become a separate crate later if needed.
This topology is a planning target, not a guarantee that exact crate names already exist.

## Runtime Flow

Phase 3 launch flow:

```text
discover artifacts
parse Vapor.toml / Rhai declarations
build artifact graph
project user/modpack-author view
run shallow metadata pre-validation
expand dependency/capability graph
run deep validation
write/update Vapor.lock and fingerprints
establish Runtime Lock for the launchable composition
handoff resolved graph/artifact to selected Engine fixture (`core_engine` plus matching `core_mod`)
run hello-world-on-steroids fixture
surface logs/output/fingerprint in launcher
shutdown cleanly
```

Launcher and SDK environments are separate graph environments from the launched composition.
Runtime Lock applies to the launchable Engine/Game composition, not to the launcher or SDK as fully dynamic Rhai-authored
runtimes.

## Work Packages

### P3-W01: Vapor Workspace Foundation

Implement the crate/workspace skeleton and shared build/test commands.

Required output:

- `vapor_core` exists unless an explicit owner-approved rename replaces it.
- `vapor_sdk` exists unless an explicit owner-approved rename replaces it.
- `vapor_launcher` exists unless an explicit owner-approved rename replaces it.
- `vapor_steam` exists unless an explicit owner-approved rename replaces it.
- `vapor_macros` exists as public API if macro-backed capability metadata is used in Phase 3.
- CI/local validation can build the Vapor workspace without requiring Steam network access.

Done means:

- The crate boundaries match the target topology pressure or deviations are documented.
- No concrete USF/worldmodel code is introduced to justify the workspace shape.

### P3-W02: Core Data Model and Manifests

Define and implement the first real data model for:

- Packagepack.
- Enginepack.
- Gamepack.
- Modpack.
- Engine Mod.
- Game Mod.
- Extension Mod.
- Source Artifact.
- Build Artifact.
- Distributable Artifact.
- Workshop item metadata.
- Capability Path and source/artifact placement metadata.
- Fingerprint.
- Vapor.toml.
- Vapor.lock.

Required output:

- Vapor.toml parser/validator.
- Vapor.lock writer/reader.
- SemVer-like version constraint parsing.
- Explicit dependency and conflict objects.
- Explicit folder/composition/storage metadata without implicit folder defaults.
- Explicit Vapor.toml coverage for every Vapor artifact root and every folder level that groups capability declarations.
- Steam/Workshop metadata fields sufficient for Phase 3 publish/install flows.

Done means:

- A Packagepack can be represented without launching an Engine/Game fixture.
- Every nested folder/root that matters is described explicitly.
- Unqualified `package` is not used as schema terminology.

### P3-W03: Capability Graph Core

Implement the minimum real capability substrate.

Required output:

- Stable path-addressed capabilities.
- Private/internal/public visibility semantics.
- Capability declarations.
- Capability Slot Types, only created by explicit opt-in.
- Capability projections for Player, Modpack Author, and Developer modes.
- Graph staging from discovery through Runtime Lock.
- Validation for cycles, missing required providers, duplicate singleton providers, explicit conflicts, version
  mismatches, invalid target roles, visibility violations, bad declarations, and fingerprint mismatches.

Done means:

- Invalid graph states hard-fail before launch.
- Diagnostics can reference both graph primitives and projected user-facing artifacts.
- Private/internal nodes do not leak into Player/Modpack Author fingerprints.

### P3-W04: Rhai Declaration Substrate

Implement Rhai as a foundational Vapor authoring substrate.

Required output:

- One Rhai file maps to one authored leaf capability declaration by default.
- Folder-level grouping/aggregation is represented explicitly through folder-level Vapor.toml and Rhai aggregation where
  needed.
- Rhai declarations can be loaded and validated without launching a concrete Engine/Game fixture.
- Declaration `ctx` is a capability graph projection.
- Callback Type, Callback Context Type, and Callback Signature are represented enough to prove one startup/logging/output
  callback flow without locking the full hook taxonomy.
- Invalid Rhai declarations produce structured diagnostics.

Done means:

- Rhai is not treated as gameplay scripting in Phase 3.
- Rhai contributes real declaration data to the Packagepack/capability graph/fingerprint path.

### P3-W05: SDK Command Surface

Implement stable public SDK command semantics before or alongside launcher UI.

Minimum command families:

- Scaffold Engine, Game, Mod, Modpack, and Packagepack projects.
- Validate local artifacts and Packagepacks.
- Print/dump fingerprints and resolved composition summaries.
- Build/package Distributable Artifacts.
- Publish/update Steam Workshop items.
- Install/update/enable/disable/uninstall Workshop-backed content.
- Validate/migrate local Vapor metadata/schema where useful for authoring; published Workshop schema migration behavior
  is deferred.

Done means:

- Commands are documented with examples.
- Commands share validation semantics with the launcher.
- Local/offline authoring works without Steam auth where Steam is not required.
- Steam-required commands fail with structured diagnostics when Steam is unavailable rather than silently degrading.

### P3-W06: Launcher Modes and UX Shell

Implement a real launcher, not just a CLI wrapper.

Required modes:

- Player Mode: select and launch Packagepacks; adjust only surface-level launch options.
- Modpack Author Mode: create/edit Packagepacks and modpack composition through public projected metadata.
- Developer Mode: inspect raw paths, Steam IDs, fingerprints, artifact roots, validation traces, and internal graph
  detail where allowed.

Required panes/surfaces:

- Installed/subscribed/local content browser.
- Packagepack composition surface.
- Validation diagnostics.
- Launcher log panel.
- Launched process log panel.
- Steam/Workshop action surface.

Done means:

- Invalid Packagepacks fail in launcher-native diagnostics before engine launch.
- Valid Packagepacks launch through the launcher and surface the fixture output/logs.

### P3-W07: Steam and Workshop Integration

Implement the Steam-exclusive public distribution rail.

Required flows:

- Steam auth/identity.
- AppID ownership/entitlement check for public/published Vapor usage.
- Workshop upload/publish.
- Workshop update.
- Private/unlisted test upload.
- Subscribe/download/install.
- Update.
- Enable/disable.
- Uninstall.
- Dependency detection and prompted auto-subscribe when SteamUGC permits it.
- Offline behavior for local authoring and already-installed content.

Done means:

- Public/published Vapor usage requires Steam identity.
- Local/offline authoring is possible.
- Steam/Workshop/network failures are structured recoverable diagnostics, not panics.
- Vapor fingerprints validate downloaded Workshop contents before use for integrity, compatibility, and reproducibility.
  This is not a claim that Vapor provides a full security sandbox or moderation layer in Phase 3.

### P3-W08: Product-Stack Fixtures

Implement deliberately simple but real executable Engine/Game fixture sets.

Required fixtures:

- Default first-party-shaped Engine fixture.
- Default first-party-shaped Game fixture.
- Alternative generic Engine fixture.
- Alternative generic Game fixture.
- At least two Engine Mods for a built-in Engine extension point.
- At least two Game Mods for a built-in Game extension point.
- At least two Extension Mods for mod-of-mod composition.
- Nested Modpack examples.

The fixtures may only produce logs, strings, files, fingerprints, and diagnostics.
They must not introduce gameplay/worldmodel/rendering/save/load semantics.
They are hello-world-on-steroids MVP fixtures, not fake placeholders.

Done means:

- Engine and Game replaceability is proven at the Vapor role level.
- Engine Mods, Game Mods, and Extension Mods all visibly affect the hello-world-on-steroids output or fingerprint.

### P3-W09: Hello-World-On-Steroids Output

Implement the observable runtime proof artifact.

Required output:

- A deterministic technical-human-readable report.
- Stable machine-readable fields.
- Selected Enginepack and Gamepack identity.
- Active mods and nested modpacks.
- Public capability paths.
- Rhai declaration identities.
- Versions and Workshop IDs where applicable.
- Fingerprint/hash summary.
- Contribution text from at least selected `core_engine`, matching `core_mod`, selected `base_mod`, Engine Mod, Game Mod,
  and Extension Mod fixture paths.

Done means:

- The output changes when composition changes.
- The output is reproducible for identical composition.
- The launcher can show the output/logs.

### P3-W10: Diagnostics and Failure Doctrine

Implement diagnostics as projections over validation truth.

Required diagnostic modes:

- Player diagnostics: what broke, which item is responsible if known, and safe actions.
- Modpack Author diagnostics: composition graph, target roles, version constraints, missing dependencies, and conflicts.
- Developer diagnostics: raw file paths, Steam IDs, fingerprints, graph paths, Rhai declaration paths, validation
  primitive names, and traces.

Required failure classes:

- Bad Vapor.toml.
- Bad Rhai declaration.
- Missing dependency.
- Version mismatch.
- Explicit conflict.
- Cycle.
- Duplicate singleton/slot provider.
- Missing required provider.
- Invalid target role.
- Visibility violation.
- Fingerprint mismatch.
- Corrupted or incomplete download.
- Steam offline/auth failed/no ownership/Workshop unavailable.
- Internal invariant violation.

Done means:

- Authoring/config errors fail before launch whenever possible.
- Internal invariants panic-fast in development builds.
- Steam/network failures remain recoverable diagnostics.

### P3-W11: Testing Suite

Implement the [[Phase 3 Vapor Testing Suite]].
This suite has three lanes: automated validation tests, local/manual scenario runs, and manual Steam/Workshop
verification records.
It is not CI-only and not merely an integration-test suite.

Required valid scenarios:

- Local-only authoring without Steam upload.
- Default unmodded Packagepack.
- Heavily modded Packagepack with Engine Mods and Game Mods.
- Extension Mod stack.
- Nested Modpack stack.
- Alternative Engine fixture.
- Alternative Game fixture.
- Local dev artifact used by an authoring Packagepack.
- Already-installed offline launch/test.
- Steam upload/update/publish roundtrip.
- Steam subscribe/download/install/update/enable/disable/uninstall roundtrip.

Required invalid scenarios:

- Missing dependency.
- Invalid version constraint.
- Duplicate singleton/slot provider.
- Missing required provider.
- Explicit conflict.
- Cycle.
- Bad Rhai declaration.
- Bad Vapor.toml.
- Corrupted or incomplete Workshop download.
- Visibility violation.
- Fingerprint mismatch.
- Player Mode blocked from unsafe local override.

Done means:

- Pure validation primitives are covered by automated unit/integration tests where practical.
- Local/manual scenario runs let the owner mix and match scenario artifacts to evaluate valid and invalid compositions.
- Steam/Workshop flows can be manually verified and recorded outside CI where live Steam is required.
- CI must not be treated as proof of live Steam integration unless live Steam verification is explicitly configured.

### P3-W12: Documentation and Phase Close

Update docs to match implementation reality.

Required output:

- SDK command examples.
- Launcher mode notes.
- Vapor.toml schema notes.
- Vapor.lock/fingerprint notes.
- Testing suite runbook.
- Steam manual verification checklist.
- Known Phase 3.5 and Phase 4 deferrals.
- Phase 4 handoff notes.

Done means:

- Phase 3 can be understood from docs without reading raw question batches first.
- Phase 4 starts from a clean Vapor base and does not need to reinterpret Phase 3 vocabulary.

## Acceptance Gate

Phase 3 is complete only when:

- A valid local Packagepack can be created, validated, fingerprinted, installed, launched, and diagnosed through Vapor
  surfaces without Steam upload.
- At least one valid publishable Packagepack path can be uploaded/updated through Steam Workshop, then downloaded,
  installed, enabled/disabled, uninstalled, and launched from installed content.
- An invalid Packagepack is blocked before launch with appropriate diagnostics.
- SDK and launcher share validation semantics.
- Steam Workshop upload/update/subscribe/download/install/enable/disable/uninstall flows work for real, even if manual
  verification is required.
- The hello-world-on-steroids fixture proves Engine/Game/mod/modpack/capability/Rhai composition without USF.
- The testing suite covers the agreed valid/invalid cases.
- No Phase 3 implementation depends on USF/worldmodel/rendering/save/load semantics.

## Explicit Deferrals

Phase 3.5 or later:

- GitHub-backed registry.
- Storefront abstraction / steam-like-platform-contracts.
- Deep launcher UX polish beyond self-documenting basics.
- Documentation generation beyond barebones support.
- Metadata schema migration for published Workshop content.
- Public hardening, proactive moderation, marketplace scale, and external security sandboxing beyond capability scoping
  and Steam identity.

Phase 4:

- USF stack.
- Worldmodel.
- Chunks.
- Metrics.
- Phenomena.
- Zones.
- Scales.
- Save/load.
- Rendering/audio/generated media.
- Gameplay simulation.
